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

struct Data {
    offset: [Scalar; 2],
    advance_size: [Scalar; 2],
    atlas_offset: [Scalar; 2],
    atlas_size: [Scalar; 2],
    texture: usize,
}

/// A struct used for caching rendered font.
pub struct GlyphCache<'a, F, T> {
    /// The font.
    pub font: rusttype::Font<'a>,
    /// The factory used to create textures.
    pub factory: F,
    /// The settings to render the font with.
    settings: TextureSettings,
    // Stores the glyph textures in atlases of 256x256.
    textures: Vec<T>,
    // The index to the current texture atlas.
    atlas: usize,
    // Texture atlas offsets from left to right.
    atlas_offsets: Vec<[u32; 2]>,
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
            textures: vec![],
            atlas: 0,
            atlas_offsets: vec![],
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
            textures: vec![],
            atlas: 0,
            atlas_offsets: vec![],
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
                texture: &self.textures[texture],
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
                    texture: &self.textures[texture],
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
                let pixel_bb_width = pixel_bounding_box.width() + 2;
                let pixel_bb_height = pixel_bounding_box.height() + 2;

                let &mut Data {
                    offset,
                    advance_size,
                    atlas_offset,
                    atlas_size,
                    texture
                } = match find_space(
                    &self.textures,
                    self.atlas,
                    &self.atlas_offsets,
                    [pixel_bb_width as u32, pixel_bb_height as u32]
                ) {
                    None => {
                        // Create a new texture atlas.
                        let mut image_buffer = Vec::<u8>::new();
                        let w = pixel_bb_width.max(256) as u32;
                        let h = pixel_bb_height.max(256) as u32;
                        image_buffer.resize((w * h) as usize, 0);
                        glyph.draw(|x, y, v| {
                            let pos = ((x + 1) + (y + 1) * w) as usize;
                            image_buffer[pos] = (255.0 * v) as u8;
                        });

                        if self.textures.len() > 0 {
                            self.atlas += 1;
                        }
                        self.atlas_offsets = vec![
                            [0, pixel_bb_height as u32],
                            [pixel_bb_width as u32, 0],
                        ];

                        let texture = self.textures.len();
                        self.textures.push({
                            if pixel_bb_width == 0 || pixel_bb_height == 0 {
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
                            atlas_size: [pixel_bb_width as Scalar, pixel_bb_height as Scalar],
                            texture
                        })
                    }
                    Some(ind) => {
                        // Use existing texture atlas.
                        let mut image_buffer = Vec::<u8>::new();
                        image_buffer.resize((pixel_bb_width * pixel_bb_height) as usize, 0);
                        glyph.draw(|x, y, v| {
                            let pos = ((x + 1) + (y + 1) * (pixel_bb_width as u32)) as usize;
                            image_buffer[pos] = (255.0 * v) as u8;
                        });

                        let texture = self.atlas;
                        let offset = self.atlas_offsets[ind];

                        // Increase y-value of atlas offsets that are matched.
                        let mut w = 0;
                        for i in ind..self.atlas_offsets.len() {
                            if self.atlas_offsets[i][1] <= offset[1] {
                                self.atlas_offsets[i][1] = offset[1] + pixel_bb_height as u32;
                            }
                            w = self.atlas_offsets[i][0] - offset[0];
                            if w >= pixel_bb_height as u32 {
                                break;
                            }
                        }
                        if w == 0 {
                            // There are no end-point atlas offset.
                            // Add new atlas offset point.
                            self.atlas_offsets.push([offset[0] + pixel_bb_width as u32, offset[1]]);
                            self.atlas_offsets.sort();
                        }

                        update_memory_alpha(
                            &mut self.textures[texture],
                            &mut self.factory,
                            &image_buffer,
                            offset,
                            [pixel_bb_width as u32, pixel_bb_height as u32],
                        )?;
                        v.insert(Data {
                            offset: [bounding_box.min.x as Scalar - 1.0,
                                     -pixel_bounding_box.min.y as Scalar + 1.0],
                            advance_size: [h_metrics.advance_width as Scalar, 0.0],
                            atlas_offset: [offset[0] as Scalar, offset[1] as Scalar],
                            atlas_size: [pixel_bb_width as Scalar, pixel_bb_height as Scalar],
                            texture
                        })
                    }
                };
                Ok(Character {
                    offset,
                    advance_size,
                    atlas_offset,
                    atlas_size,
                    texture: &self.textures[texture],
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


// Returns the index of atlas offset with room for a new glyph.
fn find_space<T: ImageSize>(
    textures: &[T],
    atlas: usize,
    atlas_offsets: &[[u32; 2]],
    size: [u32; 2]
) -> Option<usize> {
    if textures.len() == 0 {return None};

    let texture = &textures[atlas];
    let mut min: Option<usize> = None;
    'next: for i in 0..atlas_offsets.len() {
        let a = atlas_offsets[i];
        let mut nxt = [texture.get_width(), texture.get_height()];
        // Ignore next atlas offsets that have smaller y-value,
        // because they do not interfer.
        for j in i+1..atlas_offsets.len() {
            let b = atlas_offsets[j];
            nxt[0] = b[0];
            if b[1] > a[1] {break};
        }
        if nxt[0] - a[0] >= size[0] && nxt[1] - a[1] >= size[1] {
            // There is room for the glyph.
            if min.is_none() || atlas_offsets[min.unwrap()][1] > a[1] {
                // Pick the space with smallest y-value.
                min = Some(i);
            }
        }
    }
    min
}
