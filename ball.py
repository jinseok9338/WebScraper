class Ball:
    def __init__(self,canvas,**kw):
        self.canvas = canvas
        self.radius = kw.get('radius',15)
        self.pos_x = kw.get('pos_x',30)
        self.pos_y = kw.get('pos_y',30)
        self.color = kw.get('color','blue')
        self.create()


    def calculate_ball_pos(self):
        x1 = self.pos_x
        x2 = self.pos_x + self.radius
        y1 = self.pos_y
        y2 = self.pos_y + self.radius
        return x1,y1,x2,y2

    def create(self):
        coords = self.calculate_ball_pos()
        self.ball = self.canvas.create_rectangle(coords[0],coords[1],coords[2],coords[3])
        self.canvas.itemconfig(self.ball, fill=self.color)


    def move(self,x=0,y=0):
        self.pos_x += x
        self.pos_y += y
        coords = self.calculate_ball_pos()
        self.canvas.coords(self.ball,coords[0],coords[1],coords[2],coords[3])

