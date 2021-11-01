from ball import Ball

class Move(Ball):

    def __init__(self,canvas,master,**kw):
        Ball.__init__(self,canvas,**kw)
        self.master = master
        self.master.bind('w',self.keypress)
        self.master.bind('s',self.keypress)
        self.master.bind('a',self.keypress)
        self.master.bind('d',self.keypress)


    def keypress(self,event):
        """Recieve a keypress and move the ball by a specified amount"""
        print(event)
        if event.char == 'w':
            if self.collision(self.pos_x+int(self.radius/2),self.pos_y-3):
                print("collide")
            else:
                self.move(0,-2)
        elif event.char == 's':
            if self.collision(self.pos_x+int(self.radius/2),self.pos_y+self.radius+3):
                print("collide")
            else:
                self.move(0,2)
        elif event.char == 'a':
            if self.collision(self.pos_x-3,self.pos_y+int(self.radius/2)):
                print("collide")
            else:
                self.move(-2,0)
        elif event.char == 'd':
            if self.collision(self.pos_x+self.radius+3,self.pos_y+int(self.radius/2)):
                print("collide")
            else:
                self.move(2, 0)
        else:
            pass
    def collision(self,position_x,position_y):
        try:

            if self.canvas.find_closest(position_x,position_y,halo = 2)[0] != 2453:
                print(self.canvas.find_closest(position_x, position_y, halo=1))
                print("close to wall")

                return True
        except IndexError:
            print("No collision")
            print(self.canvas.find_closest(position_x, position_y, halo=1))
            return False

