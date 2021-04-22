#include <stdio.h>
//#include <conio.h>
// defines what platform
#if defined(_WIN64)
    #define PLAT "windows"
#elif defined(__linux__)
    #define PLAT "linux" 
#endif

//this gets the keycodes
int getkeyd() {
    //if (PLAT == "windows"){}
    
    int key_code = _getch();
            //printf("%d\n", key_code);


    return key_code;
    

}

int keydownd(){

    if (_kbhit()){
        return 1;
    }

    return 0;
}
int safekey(){
    if (_kbhit()){
         int key_code = _getch();
     


        return key_code;
    }
    return 0;
}