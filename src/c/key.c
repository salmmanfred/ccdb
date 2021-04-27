//#include <conio.h>
// defines what platform
#if defined(_WIN64)
    #define PLAT "windows"
    #include <stdio.h>

    /*char getkeyd() {
        //if (PLAT == "windows"){}
        
        int key_code = _getch();
                //printf("%d\n", key_code);
        return (char) key_code;
        
    }*/

    /*int keydownd(){
        if (_kbhit()){
            return 1;
        }
        return 0;
    }*/
    char getkey(){
        if (_kbhit()){
            int key_code = _getch();
        


            return key_code;
        }
        return 0;
    }
#elif defined(__linux__)
    #define PLAT "linux" 
    #include <stdio.h>
    #include <termios.h>
    #include <unistd.h>
    #include <fcntl.h>
    
    int kbhit(void)
    {
    struct termios oldt, newt;
    int ch;
    int oldf;
    
    tcgetattr(STDIN_FILENO, &oldt);
    newt = oldt;
    newt.c_lflag &= ~(ICANON | ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &newt);
    oldf = fcntl(STDIN_FILENO, F_GETFL, 0);
    fcntl(STDIN_FILENO, F_SETFL, oldf | O_NONBLOCK);
    
    ch = getchar();
    
    tcsetattr(STDIN_FILENO, TCSANOW, &oldt);
    fcntl(STDIN_FILENO, F_SETFL, oldf);
    
    if(ch != EOF)
    {
        ungetc(ch, stdin);
        return 1;
    }
    
    return 0;
    }
    
    int getkeyL()
    {
    
        if kbhit(){
            return getchar();
        }
        return 0;
        
    
    
    }
#endif