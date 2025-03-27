/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where 
    T: std::cmp::PartialOrd + std::marker::Copy
{

    fn swap<T>(array: &mut [T], a: usize, b: usize) {
        array.swap(b, a);
    }

    fn quick_sort<T>(array: &mut [T], start: usize, end: usize) 
    where 
        T: std::cmp::PartialOrd + std::marker::Copy
    {
        if start >= end {
            return;
        }

        let mut l = start + 1;
        let mut r = end;

        let value = array[start];

        while l <= r {
            while l <= r && array[l] < value {
                l += 1;
            }

            while l <= r && array[r] >= value {
                r -= 1;
            }
            if l < r {
                swap(array, l, r);
            }
        }
        swap(array, start, r);

        if r > 0 {
            quick_sort(array, start, r - 1);
        }
        
        quick_sort(array, r + 1, end);
        
    }

    quick_sort(array, 0, array.len() - 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}