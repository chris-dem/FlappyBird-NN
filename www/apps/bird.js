import p5 from 'p5';
import {PIPE_SPEED} from './pipe'

const ACC = 0.7 ;
export const WIDTH = 60;
export const HEIGHT = 35;
const LIFT = new p5.Vector(0,-20);
const MIN_Y = -10;
const MAX_Y = 30;
const ALIVE_COL = '#fae';
const DEAD_COL = '#0'
const MAX_ANGL = 2  *Math.PI/ 5 ;
const MIN_ANGL = - 2 * Math.PI / 10;

export default class Bird {
    constructor(x,y) {
        this.pos = new p5.Vector(x,y)
        this.acc = new p5.Vector(0,ACC);
        this.vel = new p5.Vector(0,0);
        this.rot = 0;
        this.state = true;
        this.color = '#fae';
        this.counter = 0;
    }

    update(p) {
        if(!this.state) {
            this.vel = PIPE_SPEED
        }
        
        // this.pos.y = this.pos.y > 
        if(this.pos.y  + HEIGHT / 2+ 10 > p.height ) {
            this.vel.y = 0;
            this.state = false
        }else{
            if(this.pos.y < 0){
                this.vel.setMag(0);
            } 
            this.vel.add(this.acc)
        }
        this.vel.y = p.constrain(this.vel.y,MIN_Y,MAX_Y);
        this.pos.add(this.vel);
           // this.rot = p.constrain(this.rot +0.2  * p.map(this.vel.y, MIN_Y,MAX_Y, MIN_ANGL,  MAX_ANGL),MIN_ANGL,MAX_ANGL);
        
    
    }

    show(p) {
        p.push()
        p.rectMode(p.CENTER); 
        p.translate(this.pos.x - WIDTH / 2 ,this.pos.y- HEIGHT / 2)
        p.fill(this.color )
        if(!this.state) {
            p.rotate(p.PI / 2);
        }
        p.rect(WIDTH / 2,HEIGHT / 2,WIDTH, HEIGHT);
        
        p.pop()
    }

    flap() {
        this.vel.add(LIFT)
    }

    setColor(s) {
        this.color = s
    }

    updateLost() {
        this.state = false;
    }

}