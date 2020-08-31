from ball import Ball


class Ray(Ball):

    def __init__(self,canvas,**kw):
        super().__init__(self,canvas,**kw)
        self.ray_dirs = [(a,0) for a in range(1001)]+[(0,a) for a in range(1001)] + [(1000,a) for a in range(1001)] +[(a,1000) for a in range(1001)]
        self.unit = 0.5

    def items_ray_collides(self,x2,y2):
        overlapping_objects = self.canvas.find_overlapping(self.pos_x, self.pos_x, x2, y2)
        overlapping_object = overlapping_objects[1]
        return overlapping_object

    def get_length_equation(self): #get equation that returns the dirs that is 2 pixels away from pos_x,pos_y
        pass


    def vector_of_a_unit(self,dirs):
        vector = self.canvas.create_line(self.pos_x,self.pos_y,int(dirs[0]),int(dirs[1])) #line should be 2 pixels
        vector.pack()





















