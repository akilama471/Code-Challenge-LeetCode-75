impl Solution {
    pub fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<i32> = Vec::new();

        for ast in asteroids {
            let mut current = ast;
            let mut destroyed = false;

            // Collision check only if stack top is moving right (positive)
            // and current asteroid is moving left (negative)
            while let Some(&top) = stack.last() {
                if top > 0 && current < 0 {
                    if top < -current {
                        // Top asteroid is smaller -> explode top
                        stack.pop();
                        continue; // Check with next top
                    } else if top == -current {
                        // Both explode
                        stack.pop();
                        destroyed = true;
                        break;
                    } else {
                        // Top asteroid survives, current is destroyed
                        destroyed = true;
                        break;
                    }
                } else {
                    // No collision scenario (same direction or away from each other)
                    break;
                }
            }

            if !destroyed {
                stack.push(current);
            }
        }

        stack
    }
}
