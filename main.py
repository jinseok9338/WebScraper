from tkinter import *
canvas_width = 1000
canvas_height = 1000

if __name__ == '__main__':
    top = Tk()

    Canvas = Canvas(top, bg="skyblue", height=canvas_height, width=canvas_width)

    Canvas.pack(fill="both", expand=True,pady=10)
    button = Button(top, overrelief="solid", width=10, repeatdelay=1000, repeatinterval=100)
    button.place(x=20,y=20)
    top.mainloop()

