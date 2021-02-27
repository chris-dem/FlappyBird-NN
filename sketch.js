function setup() {
    createCanvas(1600, 1820);
  }
  
  function draw() {
    if (mouseIsPressed) {
      fill(100);
    } else {
      fill(255);
    }
    ellipse(mouseX, mouseY, 80, 80);
  }