/*
# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    main.rs                                            :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: schongte <marvin@42.fr>                    +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2022/09/20 15:12:41 by schongte          #+#    #+#              #
#    Updated: 2022/09/20 15:16:11 by schongte         ###   ########.fr        #
#                                                                              #
# **************************************************************************** #
*/

use rand;


fn seventh()
{
    let m:f64 = 20.0; let x:f64 = 1.0; let x10 = x/m;
    println!("{}% is {}", 1.0/m, x10);
}

fn main() {
    //ex01
    let mut n = 2; n = n + 5; 

    //ex02
    let m = 10; let x:f64 = 1.0; let x10 = x*m as f64;    

    //ex03
    const N:usize = 32;
    let _arraychan:[i32;N] = [0;N];

    //ex04
    let mut x:f64;
    let M:usize = 100;
    let mut fourvec:Vec<f64> = Vec::new();
    for looper in 0..M
    {
        fourvec.push(rand::random());
    }
    //println!("{:?}", fourvec);
    
    //ex05
    let mut fivecounter:f64 = 0.0;
    for x in &fourvec
    {
        fivecounter = fivecounter + x;
    }
    println!("{}",fivecounter/M as f64);

    //ex06
    for cohn in 0..M
    {
        if cohn%4  == 0
        {
            println!("{:?}", &fourvec[cohn]);
        }

    }

    //ex07
    seventh();

    //ex08
    //
    //ex09
    println!("f32 max{} min{}", f32::MAX, f32::MIN);

    println!("f64 max{} min{}", f64::MAX, f64::MIN);

    //ex10
    #[derive(Debug, Clone)]
    struct Optimism
    {
    name : String,
    expectsal : f64,
    prob : f64,
    }

    //ex10a
    let dream = Optimism{
    name : "Nont".to_string(),
    expectsal : 0.0,
    prob : 100.0,
    };
    
    //ex10b
    fn twelth(nam: String,sal :f64,pro:f64) -> Optimism<>
    {
        return Optimism{name: nam, expectsal: sal, prob: pro,}
    }

    //ex11
    #[derive(Debug, Copy, Clone)]
    struct coordinate
    {
        coordx:f64,
        coordy:f64,
    }
    //ex11a
    const deg:char = '\u{00B0}';
    
    //ex11b
    //
    fn parser(mut k:coordinate)
    {
        let mut xig:char = 'f';
        let mut yig:char = 'u';

        if k.coordx >= 0.0 {xig = 'N'}
        else {let xig = 'S'; k.coordx = k.coordx*-1.0;}

        if k.coordy >= 0.0 {yig = 'E'}
        else {yig = 'W'; k.coordy = k.coordy*-1.0;}

        print!("({:.3}{} {}, {:.3}{} {})",k.coordx,deg,xig,k.coordy,deg,yig);
    }
    parser(coordinate{coordx:12.0, coordy:-24.0});

    //ex12
    let mut ayayaka:Vec<coordinate> = Vec::new();

    //ex12.a
    const points:usize = 5;
    const stepp:f64 = 0.01;

    for x in 0..points
    {
        ayayaka.push(coordinate{coordx: 13.725 + (stepp * x as f64), coordy: 100.776});
    }
    println!("\n");
    //ex12b
    for x in 0..points
    {
        print!("[");
        parser(ayayaka[0]);
        println!("]");
        ayayaka.remove(0);
    }
    println!("{:?}", ayayaka);



    //ex13

    struct Point{
        x:f64,
        y:f64,
    }

    struct Poly{
        sides:u64,
        pointing: Vec<Point>,
    }
    
    //ex14
    //No.

    //ex15
    let name = "<your name - Sorawis สรวิศ ソラヴィス>";
    println!("{}",name);

    //ex16
    struct info
    {
    namae: String,
    Age: usize,
    }

    //ex17
    fn holyshit(omgomgomg: char, omgomg: String) -> i64
    {
        let mut tempo = 0;
        for x in omgomg.chars(){
            tempo = tempo + 1;
            if x == omgomgomg{
                return tempo as i64;
            }
        }
        return -1;
    }
    println!("{}", holyshit('a', "Omgaomg".to_string()))
}

