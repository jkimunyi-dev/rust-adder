pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn expl(){
        assert_eq!(2+2,4)
    }

    // #[test]
    // fn fail(){
    //     panic!("Testing Fail")
    // }

    #[test]
    fn large_can_hold_smaller(){
        let larger = Rectangle{
            width : 8,
            height : 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        assert!(true)
    }

    #[test]
    fn greet_works(){
        let res = greet("Jimmie");
        assert!(res.contains("Jimmie"))
    }


}


#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}

impl Rectangle{
    fn can_hold(&self, other : &Rectangle)->bool{ // Method
        self.width > other.width && self.height > other.height
    }
} 

fn greet(name : &str) ->String{
    format!("Hello {}", name)
}