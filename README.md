# OribosLang

## example code
calculate if a number is a prime number:

    num = 9781;
    print(num);
    
    flag = false;
    if(num==1){
        print("1 is not a prime number");
    }else {
        for(i,2,num){
            if((num%2)==0){
                flag = true;
            };
        };
    
        if(flag==true){
            print("not a prime number");
        }else {
            print("prime number");
        };
    };