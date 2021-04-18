#include <stdio.h>
#define clear() printf("\033[H\033[J")
#define gotoxy(x,y) printf("\033[%d;%dH", (y), (x))


// defines platform
#if defined(_WIN64)
    #define PLAT "win"
    #include <windows.h>
    #include <conio.h>
#elif defined(__linux__)
    #define PLAT "lin" 
#endif

// changes the postion of the cursor
void xs(x,y){
        HANDLE hOut;
        COORD Position;

        hOut = GetStdHandle(STD_OUTPUT_HANDLE);

        Position.X = x;
        Position.Y = y;
        SetConsoleCursorPosition(hOut, Position);
}
// well clears the screen (not really but we dont talk about that here)
void clears() {
    if (PLAT == "lin"){
        clear();


    }else{
        xs(0,0);
        //printf("                             ");
    }
}
// moves the cursor to the x and y cordinates 
void moves(x,y){
    if (PLAT == "lin"){
        gotoxy(x,y);



    }else{
        xs(x,y);
    }
}
// hides the cursor 
void hide(){
    if (PLAT == "lin"){
    printf("\e[?25l");

    
    }else{
    HANDLE out = GetStdHandle(STD_OUTPUT_HANDLE);

    CONSOLE_CURSOR_INFO     cursorInfo;

    GetConsoleCursorInfo(out, &cursorInfo);
    cursorInfo.bVisible = 0; // set the cursor visibility
    SetConsoleCursorInfo(out, &cursorInfo);
    }
}
// shows the cursor
void show(){
    if (PLAT == "lin"){
    printf("\e[?25");

    }else{
    
    HANDLE out = GetStdHandle(STD_OUTPUT_HANDLE);

    CONSOLE_CURSOR_INFO     cursorInfo;

    GetConsoleCursorInfo(out, &cursorInfo);
    cursorInfo.bVisible = 1; // set the cursor visibility
    SetConsoleCursorInfo(out, &cursorInfo);
    }

}

