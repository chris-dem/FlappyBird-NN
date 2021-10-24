import p5 from 'p5';
export const PIPE_SPEED  = new p5.Vector(-5,0)
export const WIDTH_SIZE = 100;
export const HEIGHT_SIZE = 220;


export default class Pipe {
    constructor(x,y,init_x,color) {
        this.pos  = new p5.Vector(x,y);
        this.vel = new p5.Vector(-5,0);
        this.init_x = init_x;
        this.color = color;
    }

    update(p,x) {
        if(this.pos.x > -WIDTH_SIZE  )
            this.pos.add(this.vel);
        else {
            this.pos.x = x;
            this.pos.y = p.constrain(Math.random() * p.height, HEIGHT_SIZE,p.height - HEIGHT_SIZE - 50);
        }
    }


    show(p) {
        p.push()
        p.fill(this.color)
        p.rect(this.pos.x, 0, WIDTH_SIZE ,this.pos.y)
        p.rect(this.pos.x, this.pos.y + HEIGHT_SIZE, WIDTH_SIZE , p.height - (this.pos.y + HEIGHT_SIZE));
        p.pop()
    }
}