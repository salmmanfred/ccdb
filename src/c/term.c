

#if defined(_WIN64)
    #define PLAT "win"
    #include <windows.h>
    int *getwinsize(){
        CONSOLE_SCREEN_BUFFER_INFO csbi;
        int columns, rows;

        GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &csbi);
        columns = csbi.srWindow.Right - csbi.srWindow.Left + 1;
        rows = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;

        printf("columns: %d\n", columns);
        printf("rows: %d\n", rows);
        int x[2] = {columns, rows};
        
        
        return x;
    }
    void setwinsize(){
       // printf("\e[8;50;100t");


    }
#elif defined(__linux__)
    #define PLAT "lin" 
    #include <sys/ioctl.h>
    #include <stdio.h>
    #include <unistd.h>
    int *getwinsize(){
        int x [2];
        struct winsize w;
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &w);

      
        x[0] = w.ws_row;
        x[1] = w.ws_col;
        return x;
    }
#endif

