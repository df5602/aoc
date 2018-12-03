//! Defines a rectangle and provides various rectangle-related helpers

/// Rectangle definition
#[derive(Debug)]
pub struct Rectangle {
    /// Horizontal position of top-left corner.
    x: usize,
    /// Vertical position of top-left corner (+y points down).
    y: usize,
    /// Width of rectangle.
    width: usize,
    /// Height of rectangle.
    height: usize,
}

impl Rectangle {
    /// Create new `Rectangle`
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Returns horizontal position of top-left corner.
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns vertical position of top-left corner.
    pub fn y(&self) -> usize {
        self.y
    }

    /// Returns width of rectangle.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns height of rectangle.
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns whether `self` and `other` collide.
    pub fn collides_with(&self, other: &Rectangle) -> bool {
        other.x < self.x + self.width
            && other.x + other.width > self.x
            && other.y < self.y + self.height
            && other.y + other.height > self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collisions() {
        let rect1 = Rectangle {
            x: 3,
            y: 2,
            width: 7,
            height: 5,
        };
        let mut rect2 = Rectangle {
            x: 7,
            y: 3,
            width: 7,
            height: 7,
        };

        assert!(rect1.collides_with(&rect2));

        rect2.x = 9;
        assert!(rect1.collides_with(&rect2));

        rect2.x = 10;
        assert!(!rect1.collides_with(&rect2));

        rect2.x = 3;
        assert!(rect1.collides_with(&rect2));

        rect2.x = 2;
        assert!(rect1.collides_with(&rect2));

        rect2.x = 1;
        rect2.width = 2;
        assert!(!rect1.collides_with(&rect2));

        rect2.y = 7;
        rect2.width = 7;

        rect2.y = 6;
        assert!(rect1.collides_with(&rect2));

        rect2.y = 7;
        assert!(!rect1.collides_with(&rect2));

        rect2.y = 1;
        rect2.height = 2;
        assert!(rect1.collides_with(&rect2));

        rect2.height = 1;
        assert!(!rect1.collides_with(&rect2));
    }
}
