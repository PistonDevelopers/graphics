//! Texture packing.

use crate::ImageSize;

/// A texture packer using a skyline heuristic.
///
/// For offline texture packing, see [texture_packer](https://github.com/pistondevelopers/texture_packer).
///
/// Designed for adding textures one by one to current texture atlas.
/// Packs tiles without backtracking or knowledge about future tiles.
///
/// - Perfect at packing tiles of same size
/// - Good at packing tiles of some unit size
/// - Decent at packing tiles of similar sizes
/// - Can be used with pre-sorted tile sizes for better packing
///
/// Can also be used as storage for textures.
///
/// ### Design
///
/// A skyline is a list of non-hole atlas offsets,
/// used to efficiently determine a good place to put the next tile.
///
/// In this texture packer,
/// only a single skyline is kept track of,
/// since new texture atlases are created by need.
///
/// This texture packer has runtime complexity `O(N^2)` for inserting a new tile,
/// where `N` is the number of points in the skyline.
/// Since `N` is usually a low number, the packing is pretty fast.
///
/// The algorithm was designed by Sven Nilsen (2019) for Piston-Graphics.
pub struct TexturePacker<T> {
    /// Stores current texture atlas and previously created ones.
    pub textures: Vec<T>,
    /// The index to the current texture atlas.
    pub atlas: usize,
    /// Texture atlas offsets from left to right.
    ///
    /// When a new tile is added with same offset,
    /// it updates the atlas offsets that it overlaps.
    /// This means that "holes" get filled in over time.
    pub skyline: Vec<[u32; 2]>,
}

impl<T: ImageSize> TexturePacker<T> {
    /// Returns a new `TexturePacker`.
    pub fn new() -> TexturePacker<T> {
        TexturePacker {
            textures: vec![],
            atlas: 0,
            skyline: vec![],
        }
    }

    /// Create a new texture atlas with an initial tile.
    ///
    /// The new texture atlas is made the current one.
    pub fn create(&mut self, size: [u32; 2], texture: T) -> usize {
        let id = self.textures.len();
        if !self.textures.is_empty() {
            self.atlas += 1;
        }
        self.skyline = vec![[0, size[1]], [size[0], 0]];
        self.textures.push(texture);
        id
    }

    /// Update current texture atlas.
    ///
    /// - ind: index of atlas offset in the skyline
    /// - size: size of new tile
    ///
    /// Returns the index of the current texture atlas and
    /// the atlas offset of the new tile.
    pub fn update(&mut self, ind: usize, size: [u32; 2]) -> (usize, [u32; 2]) {
        let texture = self.atlas;
        let offset = self.skyline[ind];

        // Increase y-value of atlas offsets that are matched.
        let mut w = 0;
        for i in ind..self.skyline.len() {
            if self.skyline[i][1] <= offset[1] {
                self.skyline[i][1] = offset[1] + size[1];
            }
            w = self.skyline[i][0] - offset[0];
            if w >= size[1] {
                break;
            }
        }
        if w == 0 {
            // There is no end-point atlas offset.
            // Add new atlas offset point.
            self.skyline.push([offset[0] + size[0], offset[1]]);
            self.skyline.sort();
        }

        (texture, offset)
    }

    /// Returns the index of atlas offset in skyline with room for a new tile.
    ///
    /// Returns `None` if no room was found in the current texture atlas.
    pub fn find_space(&self, size: [u32; 2]) -> Option<usize> {
        if self.textures.is_empty() {
            return None;
        };

        let texture = &self.textures[self.atlas];
        let mut min: Option<(usize, u32)> = None;
        for i in 0..self.skyline.len() {
            let a = self.skyline[i];
            let mut nxt = [texture.get_width(), texture.get_height()];
            // Ignore next atlas offsets that have smaller y-value,
            // because they do not interfer.
            for j in i + 1..self.skyline.len() {
                let b = self.skyline[j];
                nxt[0] = b[0];
                if b[1] > a[1] {
                    break;
                };
            }
            if nxt[0] - a[0] >= size[0] && nxt[1] - a[1] >= size[1] {
                // There is room for the glyph.
                if min.is_none()
                    || min.unwrap().1 > nxt[0] - a[0]
                    || self.skyline[min.unwrap().0][1] > a[1]
                {
                    // Pick the space with smallest y-value.
                    min = Some((i, nxt[0] - a[0]));
                }
            }
        }
        min.map(|n| n.0)
    }
}
