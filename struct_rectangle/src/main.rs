//STRUCT
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    //CREAT RECTANGLE
    let rect: Rectangle = Rectangle {
        height: 20,
        width: 15,
    };

    let react_area = area(&rect);
    println!("Area of Rectangle = {}", react_area);

    let react_perimeter = perimeter(&rect);
    println!("Perimeter of Rectangle = {}", react_perimeter);
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

fn perimeter(rect: &Rectangle) -> u32 {
    2 * (rect.height + rect.width)
}
