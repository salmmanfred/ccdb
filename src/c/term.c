

#if defined(_WIN64)
    #define PLAT "win"
    #include <windows.h>
    int getwinsizeROW(){
        CONSOLE_SCREEN_BUFFER_INFO csbi;
        int rows;

        GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &csbi);
        
        rows = csbi.srWindow.Bottom - csbi.srWindow.Top + 1;

        
        
        return rows;
    }
    int getwinsizeCOL(){
        CONSOLE_SCREEN_BUFFER_INFO csbi;
        int columns;

        GetConsoleScreenBufferInfo(GetStdHandle(STD_OUTPUT_HANDLE), &csbi);
        columns = csbi.srWindow.Right - csbi.srWindow.Left + 1;
      
        
        
        return columns;
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

