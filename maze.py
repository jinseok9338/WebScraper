import random
from tkinter import *


class Cell(Frame):
    """A cell in the maze.

    A maze "Cell" is a point in the grid which may be surrounded by walls to
    the north, east, south or west.

    """
    # A wall separates a pair of cells in the N-S or W-E directions.
    wall_pairs = {'N': 'S', 'S': 'N', 'E': 'W', 'W': 'E'}

    def __init__(self, x, y):
        """Initialize the cell at (x,y). At first it is surrounded by walls."""

        self.x, self.y = x, y
        self.walls = {'N': True, 'S': True, 'E': True, 'W': True}

    def has_all_walls(self):
        """Does this cell still have all its walls?"""
        return all(self.walls.values())

    def knock_down_wall(self, other, wall):
        """Knock down the wall between cells self and other."""
        self.walls[wall] = False
        other.walls[Cell.wall_pairs[wall]] = False

class Maze(Frame):
    """A Maze, represented as a grid of cells."""

    def __init__(self, nx, ny, ix=0, iy=0):
        """Initialize the maze grid.
        The maze consists of nx x ny cells and will be constructed starting
        at the cell indexed at (ix, iy).

        """
        self.nx, self.ny = nx, ny
        self.ix, self.iy = ix, iy
        self.maze_map = [[Cell(x, y) for y in range(ny)] for x in range(nx)]
        super(Maze, self).__init__()



    def cell_at(self, x, y):
        """Return the Cell object at (x,y)."""

        return self.maze_map[x][y]

    def __str__(self):
        """Return a (crude) string representation of the maze."""

        maze_rows = ['-' * self.nx * 2]
        for y in range(self.ny):
            maze_row = ['|']
            for x in range(self.nx):
                if self.maze_map[x][y].walls['E']:
                    maze_row.append(' |')
                else:
                    maze_row.append('  ')
            maze_rows.append(''.join(maze_row))
            maze_row = ['|']
            for x in range(self.nx):
                if self.maze_map[x][y].walls['S']:
                    maze_row.append('-+')
                else:
                    maze_row.append(' +')
            maze_rows.append(''.join(maze_row))
        return '\n'.join(maze_rows)

    def find_valid_neighbours(self, cell):
        """Return a list of unvisited neighbours to cell."""

        delta = [('W', (-1,0)),
                 ('E', (1,0)),
                 ('S', (0,1)),
                 ('N', (0,-1))]
        neighbours = []
        for direction, (dx,dy) in delta:
            x2, y2 = cell.x + dx, cell.y + dy
            if (0 <= x2 < self.nx) and (0 <= y2 < self.ny):
                neighbour = self.cell_at(x2, y2)
                if neighbour.has_all_walls():
                    neighbours.append((direction, neighbour))
        return neighbours

    def make_maze(self):
        # Total number of cells.
        n = self.nx * self.ny
        cell_stack = []
        current_cell = self.cell_at(self.ix, self.iy)
        # Total number of visited cells during maze construction.
        nv = 1

        while nv < n:
            neighbours = self.find_valid_neighbours(current_cell)

            if not neighbours:
                # We've reached a dead end: backtrack.
                current_cell = cell_stack.pop()
                continue

            # Choose a random neighbouring cell and move to it.
            direction, next_cell = random.choice(neighbours)
            current_cell.knock_down_wall(next_cell, direction)
            cell_stack.append(current_cell)
            current_cell = next_cell
            nv += 1

    def draw_maze(self, xp, yp):
        self.pack(fill=BOTH, expand=1)
        line_width = 5

        cell_line_length = 50
        canvas = Canvas(self)
        cell_starting_points = [[(x, y) for y in range(yp, yp + cell_line_length * self.ny, cell_line_length)] for x in
                                range(xp, xp + cell_line_length * self.ny, cell_line_length)]

        print(cell_starting_points)
        #Something wrong with drawing line but it packs at least

        for a in range(self.nx):
            for b in range(self.ny):
                if self.maze_map[a][b].walls["N"]:
                    print(a,b,"North")
                    canvas.create_line(cell_starting_points[a][b][0], cell_starting_points[a][b][1],
                                       cell_starting_points[a][b][0] + cell_line_length, cell_starting_points[a][b][1])

                if self.maze_map[a][b].walls["S"]:
                    print(a,b,"South")
                    canvas.create_line(cell_starting_points[a][b][0], cell_starting_points[a][b][1] + cell_line_length,
                                       cell_starting_points[a][b][0] + cell_line_length,
                                       cell_starting_points[a][b][1] + cell_line_length)

                if self.maze_map[a][b].walls["E"]:
                    print(a,b,"East")
                    canvas.create_line(cell_starting_points[a][b][0]+ cell_line_length, cell_starting_points[a][b][1],
                                       cell_starting_points[a][b][0]+ cell_line_length, cell_starting_points[a][b][1]+ cell_line_length )

                if self.maze_map[a][b].walls["W"]:
                    print(a,b,"West")
                    canvas.create_line(cell_starting_points[a][b][0] , cell_starting_points[a][b][1],
                                       cell_starting_points[a][b][0],
                                       cell_starting_points[a][b][1] + cell_line_length)


        canvas.pack(fill=BOTH, expand=1)

        return cell_starting_points









