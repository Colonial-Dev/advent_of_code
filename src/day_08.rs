//! # Day 8 - Treetop Tree House
//! 
//! Puzzle opened on time. 
//! - P1 completed @ 00:29:05 (5330)
//! - P2 completed @ 00:36:48 (3148)
//! 
//! What even is this schizo difficulty curve? Not that I'm complaining about an easy problem after
//! yesterday, LOL.
//! 
//! ... I still hate working with 2D arrays though.
//! 
//! ## Parsing
//! As aformentioned, this problem basically screams "parse me into a 2D array" - so that's exactly what I did,
//! additionally using [`char::to_digit`] to convert the tree heights to a [`Vec<Vec<u32>>`] (aka a [`Vec2D<u32>`]).
//! 
//! ## Solutions
//! My original approach was just to write some functions that used `for` loops to step horizontally and vertically
//! through the map and compute an answer (either a [`bool`] visibility or a [`usize`] score.)
//! 
//! This *worked*, and was actually pretty fast because *mumble mumble* linear access patterns *mumble* CPU cache *mumble*, but 
//! it was also ugly as sin. So I decided to spend the time to make it a lot more functional-style (i.e. pretty and declarative
//! with lots of iterators.)
//! 
//! To make this happen, the main thing I would need would be a way to get all the elements to the north/south/whatever of a given
//! element in our 2D array *as an iterator*, so we can go wild with FP and banish all (well, most) of the imperative `for` loops back
//! to the medieval land of C where they belong.
//! 
//! There are a few ways to handle this, but I chose the more ergonomic path by defining a *trait* - [`Grid<T>`] - to extend [`Vec2D<T>`] with.
//! The interface of [`Grid<T>`] exposes:
//! - [`north_of(row, col)`](Grid::north_of) - (as well as [`south_of`](Grid::south_of) and so on.) These methods receive the row and column indices
//! of an element in the underlying grid structure and yield an iterator over all the elements to its north/south/east/west.
//! - [`enumerate_coordinates`](Grid::enumerate_coordinates) - yields an ordered iterator over all possible coordinate pairs within the grid, in the form of (row, column).
//! - [`is_on_edge`](Grid::is_on_edge) - receives the row and column indices of an element and evaluates whether or not it is on the "edge" of the grid.
//! 
//! With the [`Grid<T>`] trait defined, I then provided a blanket implementation of the trait for all [`Vec<Vec<T>>`]/[`Vec2D<T>`] (with T being totally unbounded beyond
//! the implicit [`Sized`] bound.) Now all the functionality of [`Grid<T>`] is directly available as methods on any 2D array - once [`Grid<T>`] is in scope, of course.
//! 
//! This is a very convenient; the only downside is that (due to the limitations of trait methods) the returned iterators must incur the costs of being 
//! [`Box`]ed (heap allocated) and dynamically dispatched rather than stack-allocated and statically dispatched.
//! 
//! Now that we have a good interface to our tree height map, computing the actual solutions is relatively simple. 
//! 
//! For part one, we can use [`Grid::enumerate_coordinates`] and filter using the following algorithm:
//! 1. If the coordinate is on the edge, keep it and continue.
//! 2. Otherwise, we need to scan along the axes of the coordinate to determine its visibility. 
//! 3. We create a `[bool; 4]` to track which directions (if any) the current tree is visible from;
//! then, we go through [`Grid::north_of`], [`Grid::south_of`] and so on, using the [`Iterator::any`] adaptor
//! to scan the yielded iterators and see if any of the trees they contain are taller than the one we're currently inspecting, storing
//! the result in the aformentioned bool array.
//! 4. If any of the elements in the bool array are true, the tree is visible and we should keep its coordinates; otherwise, throw them out.
//! 
//! Once that filtering is done, the [`Iterator::count`] consumer is all we need to get the answer.
//! 
//! Part two starts the same way by enumerating over the grid's coordinates, but uses a more complex algorithm
//! to compute the scenic score of each tree:
//! 1. Initialize a `[u32; 4]` array to track the score in each cardinal direction.
//! 2. Define a `filter` closure that takes a boolean state and a `u32` reference - we'll use this with
//! [`Iterator::scan`] later: 
//!    - It returns `None` if the boolean state is true. 
//!    - If the `u32` is greater than or equal to the height of the tree we're currently inspecting, it sets
//!      the boolean state to true.
//!    - Yield `Some(<dereferenced u32>)`.
//! 3. Once again going through [`Grid`]'s cardinal iterator methods, we use [`Iterator::scan`] on them
//! with an initial state of `false` and the `filter` closure we just defined. This will remove all items *past*
//! the tree blocking visibility, which is why we didn't use something like [`Iterator::take_while`] - they don't yield
//! the item that caused them to terminate.
//! 4. Invoke the [`Iterator::count`] consumer and store its result in the aformentioned [`u32`] array.
//! 5. Use `array.iter().product()` to determine the final scenic score for the current tree.
//! 
//! Once the above has been applied to all coordinates, a simple call to [`Iterator::max`] is all it takes to get an answer.
use super::*;

impl Solution<DAY_08> for Solutions {
    type Input<'i> = Vec2D<u32>;
    type Output = usize;

    fn parse(puzzle: &str) -> Self::Input<'_> {
        let mut map = Vec::new();
        puzzle
            .lines()
            .for_each(|line| {
                let chars: Vec<_> = line
                    .chars()
                    .map(|char| char.to_digit(10))
                    .map(Option::unwrap)
                    .collect();
                
                map.push(chars);
            });
        map
    }

    fn part_one(input: &Self::Input<'_>) -> Option<Self::Output> {
        input
            .enumerate_coordinates()
            .filter(|coordinates| {
                let (row, col) = *coordinates;

                if input.is_on_edge(row, col) {
                    return true;
                }
            
                let mut visible = [true; 4];
                let root_height = input[row][col];
    
                visible[0] = !input.north_of(row, col).any(|x| *x >= root_height);
                visible[1] = !input.south_of(row, col).any(|x| *x >= root_height);
                visible[2] = !input.east_of(row, col).any(|x| *x >= root_height);
                visible[3] = !input.west_of(row, col).any(|x| *x >= root_height);
    
                visible.iter().any(|vis| *vis)
            })           
            .count()
            .into()
    }

    fn part_two(input: &Self::Input<'_>) -> Option<Self::Output> {
        input
            .enumerate_coordinates()
            .map(|(row, col)| {
                let mut scores = [0; 4];
                let root_height = input[row][col];

                let filter = |blocked: &mut bool, x: &u32| {
                    if *blocked { return None }
                    else if *x >= root_height { *blocked = true }
                    Some(*x)
                };

                scores[0] = input.north_of(row, col).scan(false, filter).count();
                scores[1] = input.south_of(row, col).scan(false, filter).count();
                scores[2] = input.east_of(row, col).scan(false, filter).count();
                scores[3] = input.west_of(row, col).scan(false, filter).count();

                scores.iter().product::<usize>()
            })
            .max()
    }
}

impl Test<DAY_08> for Solutions {
    fn expected(part: bool) -> Self::Output {
        match part {
            PART_ONE => 21,
            PART_TWO => 8
        }
    }
}

type Vec2D<T> = Vec<Vec<T>>;

trait Grid<T> {
    fn north_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_>;
    fn south_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_>;
    fn east_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_>;
    fn west_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_>;
    fn enumerate_coordinates(&self) -> Box<dyn Iterator<Item=(usize, usize)>>;
    fn is_on_edge(&self, row: usize, col: usize) -> bool;
}

impl<T> Grid<T> for Vec2D<T> {
    fn north_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_> {
        let range = 0..row;
        let iterator = range
            .rev()
            .map(move |i| &self[i][col]);
        
        Box::new(iterator)
    }

    fn south_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_> {
        let range = (row + 1)..self[col].len();
        let iterator = range
            .into_iter()
            .map(move |i| &self[i][col]);
        
        Box::new(iterator)
    }

    fn east_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_> {
        let range = (col + 1)..self[row].len();
        let iterator = range
            .into_iter()
            .map(move |i| &self[row][i]);

        Box::new(iterator)
    }

    fn west_of(&self, row: usize, col: usize) -> Box<dyn Iterator<Item = &T> + '_> {
        let range = 0..col;
        let iterator = range
            .rev()
            .map(move |i| &self[row][i]);
        
        Box::new(iterator)
    }

    fn enumerate_coordinates(&self) -> Box<dyn Iterator<Item=(usize, usize)>> {
        let mut coordinates = Vec::new();

        for (index, vec) in self.iter().enumerate() {
            for i in 0..vec.len() {
                coordinates.push((index, i))
            }
        }

        Box::new(coordinates.into_iter())
    }

    fn is_on_edge(&self, row: usize, col: usize) -> bool {
        row == 0 || row == self.len() - 1
        ||
        col == 0 || col == self[row].len() - 1
    }
}

derive_tests!(Solutions, DAY_08);