const dump = `CE ED 66 66 CC 0D 00 0B 03 73 00 83 00 0C 00 0D
              00 08 11 1F 88 89 00 0E DC CC 6E E6 DD DD D9 99
              BB BB 67 63 6E 0E EC CC DD DC 99 9F BB B9 33 3E`;

function isPowerOfTwo(x) {
    return (x != 0) && ((x & (x - 1)) == 0);
}

function toBin(hex){
    return (parseInt(hex, 16).toString(2)).padStart(8, '0');
}

function hexString2bin(hex){
    const seq = hex.replace(/\W|\n|\r/g, "");
    if(!isPowerOfTwo(seq)) return;
    let binary = "";
    for(let i = 0; i < seq.length; i+=2) {
        const n = seq.substring(i, i+2);
        const bin = toBin(n);
        binary += bin;
    }
    return binary;
}

function drawBitmap(bin, width = 48) {
    const bits = bin.split("");

    const canv = document.createElement("canvas");
    const ctxt = canv.getContext("2d");

    const pp = 24;

    canv.width = width * pp;
    canv.height = (bin.length / width) * pp;

    let i = 0;
    for(let bit of bits) {
        if(+bit === 1) {
            const tx = i % 4;
            const ty = Math.floor(i / 4);

            const y = ty - (Math.floor(i / (4 * 4)) * 4) + (Math.floor(i / (48 * 4)) * 4);
            const x = tx + (Math.floor(i / (4 * 4)) * 4) - (Math.floor(i / (48 * 4)) * 48);

            ctxt.fillStyle = `hsl(${(x / width) * 255}, 75%, 50%)`;
            ctxt.fillRect(x * pp, y * pp, 1 * pp, 1 * pp);
        }
        i++;
    }

    return canv;
}

const logoBitmap = hexString2bin(dump);
const canv = drawBitmap(logoBitmap);

canv.style.position = "fixed";
canv.style.zIndex = "100000";
canv.style.top = "10px";
canv.style.left = "10px";
canv.style.imageRendering = "pixelated";
canv.style.width = "900px";

document.body.append(canv);
