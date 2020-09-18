use std::io;
fn main() {
    let mut contx=1;
    let mut cont=1;
    let mut resultado=String::new();
    let unidade = vec!["".to_string(),"I".to_string(),"II".to_string(),"III".to_string(),"IV".to_string(),"V".to_string(),"VI".to_string(),"VII".to_string(),"VIII".to_string(),"IX".to_string(),"X".to_string()];
    let decimal = vec!["".to_string(),"X".to_string(),"XX".to_string(),"XXX".to_string(),"XL".to_string(),"L".to_string(),"LX".to_string(),"LXX".to_string(),"LXX".to_string(),"XC".to_string(),"C".to_string()];
    let centena = vec!["".to_string(),"C".to_string(),"CC".to_string(),"CCC".to_string(),"CD".to_string(),"D".to_string(),"DC".to_string(),"DCC".to_string(),"DCCC".to_string(),"CM".to_string(),"M".to_string()];
    let milhar = vec!["".to_string(),"M".to_string(),"MM".to_string(),"MMM".to_string()];
    let mut lista_romana = Vec::new();
    println!("Please input your guess.");

    let mut x = String::new();

    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    let mut choice:usize = x
                    .trim()
                    .parse()
                    .expect("Opção incorreta");


    while contx <5  {
        if cont==1 && contx==1{
            
            lista_romana.push(&unidade[choice%10]);
            println!("unidade {}",&choice);
        } if cont==2 && contx==2{

            lista_romana.push(&decimal[choice%10]);
            println!("dezana {}",&choice);

        }if cont==3 && contx==3{
            lista_romana.push(&centena[choice%10]);
            println!("centena {}",&choice);

        }if cont==4 && contx==4{
            lista_romana.push(&milhar[choice%10]);
            println!("milher {}",&choice);
        }if choice < 10 {
            break;
            
            
        }
        choice = choice/10;
        contx+=1;
        cont += 1;
    }
        
    let mut r = lista_romana.iter()
                                .rev()
                                .fold(String::new(), |r, c| r + c.as_str() + " ");
    r.pop();
    println!("{:?}",r);

}

