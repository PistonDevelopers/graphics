//! Glyph caching using the RustType library.

extern crate rusttype;
use texture::{ops, CreateTexture, UpdateTexture, Format, TextureSettings};
use std::collections::HashMap;

extern crate fnv;
use self::fnv::FnvHasher;
use std::hash::BuildHasherDefault;

use std::path::Path;
use std::io::Read;
use std::fs::File;

use ImageSize;
use types::{FontSize, Scalar};
use character::{Character, CharacterCache};
use texture_packer::TexturePacker;

struct Data {
    offset: [Scalar; 2],
    advance_size: [Scalar; 2],
    atlas_offset: [Scalar; 2],
    atlas_size: [Scalar; 2],
    texture: usize,
}

/// The minimum atlas size.
pub const ATLAS_SIZE: [u32; 2] = [256; 2];

/// A struct used for caching rendered font.
pub struct GlyphCache<'a, F, T> {
    /// The font.
    pub font: rusttype::Font<'a>,
    /// The factory used to create textures.
    pub factory: F,
    /// The settings to render the font with.
    settings: TextureSettings,
    texture_packer: TexturePacker<T>,
    // Maps from fontsize and character to offset, texture offset, advance size and texture index.
    data: HashMap<(FontSize, char), Data, BuildHasherDefault<FnvHasher>>,
}

impl<'a, F, T> GlyphCache<'a, F, T>
    where T: CreateTexture<F> +
          UpdateTexture<F, Error = <T as CreateTexture<F>>::Error> +
          ImageSize
{
    /// Constructs a GlyphCache from a Font.
    pub fn from_font(font: rusttype::Font<'a>, factory: F, settings: TextureSettings) -> Self {
        let fnv = BuildHasherDefault::<FnvHasher>::default();
        GlyphCache {
            font: font,
            factory: factory,
            settings: settings,
            texture_packer: TexturePacker::new(),
            data: HashMap::with_hasher(fnv),
        }
    }

    /// Constructor for a GlyphCache.
    pub fn new<P>(font: P,
                  factory: F,
                  settings: TextureSettings)
                  -> ::std::io::Result<GlyphCache<'static, F, T>>
        where P: AsRef<Path>
    {
        let fnv = BuildHasherDefault::<FnvHasher>::default();
        let mut file = File::open(font)?;
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer)?;

        let collection = rusttype::FontCollection::from_bytes(file_buffer);
        let font = collection.unwrap().into_font().unwrap();
        Ok(GlyphCache {
            font: font,
            factory: factory,
            settings: settings,
            texture_packer: TexturePacker::new(),
            data: HashMap::with_hasher(fnv),
        })
    }

    /// Creates a GlyphCache for a font stored in memory.
    pub fn from_bytes(font: &'a [u8],
                      factory: F,
                      settings: TextureSettings)
                      -> Result<GlyphCache<'a, F, T>, ()> {
        let collection = rusttype::FontCollection::from_bytes(font).or(Err(()))?;
        let font = collection.into_font().or(Err(()))?;
        Ok(Self::from_font(font, factory, settings))
    }

    /// Load all characters in the `chars` iterator for `size`
    pub fn preload_chars<I>(
        &mut self,
        size: FontSize,
        chars: I
    ) -> Result<(), <T as CreateTexture<F>>::Error>
        where I: Iterator<Item = char>
    {
        for ch in chars {
            self.character(size, ch)?;
        }
        Ok(())
    }

    /// Load all the printable ASCII characters for `size`. Includes space.
    pub fn preload_printable_ascii(
        &mut self,
        size: FontSize
    ) -> Result<(), <T as CreateTexture<F>>::Error> {
        // [0x20, 0x7F) contains all printable ASCII characters ([' ', '~'])
        self.preload_chars(size, (0x20u8..0x7F).map(|ch| ch as char))
    }

    /// Return `ch` for `size` if it's already cached. Don't load.
    /// See the `preload_*` functions.
    pub fn opt_character(&self, size: FontSize, ch: char) -> Option<Character<T>> {
        self.data.get(&(size, ch)).map(|&Data {
            offset, advance_size, atlas_offset, atlas_size, texture
        }| {
            Character {
                offset,
                advance_size,
                atlas_offset,
                atlas_size,
                texture: &self.texture_packer.textures[texture],
            }
        })
    }
}

impl<'b, F, T: ImageSize> CharacterCache for GlyphCache<'b, F, T>
    where T: CreateTexture<F> +
             UpdateTexture<F, Error = <T as CreateTexture<F>>::Error>
{
    type Texture = T;
    type Error = <T as CreateTexture<F>>::Error;

    fn character<'a>(&'a mut self,
                     size: FontSize,
                     ch: char)
                     -> Result<Character<'a, T>, Self::Error> {
        use std::collections::hash_map::Entry;
        use self::rusttype as rt;

        let size = ((size as f32) * 1.333).round() as u32; // convert points to pixels

        match self.data.entry((size, ch)) {
            //returning `into_mut()' to get reference with 'a lifetime
            Entry::Occupied(v) => {
                let &mut Data {offset, advance_size, atlas_offset, atlas_size, texture} = v.into_mut();
                Ok(Character {
                    offset,
                    advance_size,
                    atlas_offset,
                    atlas_size,
                    texture: &self.texture_packer.textures[texture],
                })
            }
            Entry::Vacant(v) => {
                // this is only None for invalid GlyphIds,
                // but char is converted to a Codepoint which must result in a glyph.
                let glyph = self.font.glyph(ch);
                let scale = rt::Scale::uniform(size as f32);
                let mut glyph = glyph.scaled(scale);

                // some fonts do not contain glyph zero as fallback, instead try U+FFFD.
                if glyph.id() == rt::GlyphId(0) && glyph.shape().is_none() {
                    glyph = self.font.glyph('\u{FFFD}').scaled(scale);
                }

                let h_metrics = glyph.h_metrics();
                let bounding_box = glyph.exact_bounding_box().unwrap_or(rt::Rect {
                    min: rt::Point { x: 0.0, y: 0.0 },
                    max: rt::Point { x: 0.0, y: 0.0 },
                });
                let glyph = glyph.positioned(rt::point(0.0, 0.0));
                let pixel_bounding_box = glyph.pixel_bounding_box().unwrap_or(rt::Rect {
                    min: rt::Point { x: 0, y: 0 },
                    max: rt::Point { x: 0, y: 0 },
                });
                let size = [
                    (pixel_bounding_box.width() + 2) as u32,
                    (pixel_bounding_box.height() + 2) as u32,
                ];

                let &mut Data {
                    offset,
                    advance_size,
                    atlas_offset,
                    atlas_size,
                    texture
                } = match self.texture_packer.find_space(size) {
                    None => {
                        // Create a new texture atlas.
                        let mut image_buffer = Vec::<u8>::new();
                        let w = size[0].max(ATLAS_SIZE[0]) as u32;
                        let h = size[1].max(ATLAS_SIZE[1]) as u32;
                        image_buffer.resize((w * h) as usize, 0);
                        glyph.draw(|x, y, v| {
                            let pos = ((x + 1) + (y + 1) * w) as usize;
                            image_buffer[pos] = (255.0 * v) as u8;
                        });

                        let texture = self.texture_packer.create(size, {
                            if size[0] == 0 || size[1] == 0 {
                                empty(&mut self.factory, &self.settings)?
                            } else {
                                from_memory_alpha(&mut self.factory,
                                                  &image_buffer,
                                                  [w, h],
                                                  &self.settings)?
                            }
                        });
                        v.insert(Data {
                            offset: [bounding_box.min.x as Scalar - 1.0,
                                     -pixel_bounding_box.min.y as Scalar + 1.0],
                            advance_size: [h_metrics.advance_width as Scalar, 0.0],
                            atlas_offset: [0.0; 2],
                            atlas_size: [size[0] as Scalar, size[1] as Scalar],
                            texture
                        })
                    }
                    Some(ind) => {
                        // Use existing texture atlas.
                        let mut image_buffer = Vec::<u8>::new();
                        image_buffer.resize((size[0] * size[1]) as usize, 0);
                        glyph.draw(|x, y, v| {
                            let pos = ((x + 1) + (y + 1) * size[0]) as usize;
                            image_buffer[pos] = (255.0 * v) as u8;
                        });

                        let (texture, offset) = self.texture_packer.update(ind, size);

                        update_memory_alpha(
                            &mut self.texture_packer.textures[texture],
                            &mut self.factory,
                            &image_buffer,
                            offset,
                            size,
                        )?;
                        v.insert(Data {
                            offset: [bounding_box.min.x as Scalar - 1.0,
                                     -pixel_bounding_box.min.y as Scalar + 1.0],
                            advance_size: [h_metrics.advance_width as Scalar, 0.0],
                            atlas_offset: [offset[0] as Scalar, offset[1] as Scalar],
                            atlas_size: [size[0] as Scalar, size[1] as Scalar],
                            texture
                        })
                    }
                };
                Ok(Character {
                    offset,
                    advance_size,
                    atlas_offset,
                    atlas_size,
                    texture: &self.texture_packer.textures[texture],
                })
            }
        }
    }
}

fn empty<F, T: CreateTexture<F>>(factory: &mut F,
                                 settings: &TextureSettings)
                                 -> Result<T, T::Error> {
    CreateTexture::create(factory, Format::Rgba8, &[0u8; 4], [1, 1], settings)
}

fn from_memory_alpha<F, T: CreateTexture<F>>(factory: &mut F,
                                             buf: &[u8],
                                             size: [u32; 2],
                                             settings: &TextureSettings)
                                             -> Result<T, T::Error> {
    let buffer: Vec<u8> = ops::alpha_to_rgba8(buf, size);
    CreateTexture::create(factory, Format::Rgba8, &buffer, size, settings)
}

fn update_memory_alpha<F, T: UpdateTexture<F>>(
                                            texture: &mut T,
                                            factory: &mut F,
                                             buf: &[u8],
                                             offset: [u32; 2],
                                             size: [u32; 2])
                                             -> Result<(), T::Error> {
    let buffer: Vec<u8> = ops::alpha_to_rgba8(buf, size);
    texture.update(factory, Format::Rgba8, &buffer, offset, size)
}
