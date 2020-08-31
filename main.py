import tkinter as tk
from ball import Ball
from move import Move
from maze import Maze, Cell
root = tk.Tk()
root.title("Test")
root.geometry("1000x1000")

canvas = tk.Canvas(root, width=1000, height=1000, borderwidth=0, highlightthickness=0, bg="white")
canvas.grid()


maze =Maze(35,35,canvas)
maze.make_maze()
maze.draw_maze(30,30)
move= Move(canvas,root) #ball id = 2453

root.mainloop()

