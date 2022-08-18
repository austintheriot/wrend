// web_sys does not provide this functionality out of the box 
// (perhaps because of low-ish browser support?)
// so it must be implemented manually here in JavaScript
export function captureStreamFromCanvas(canvas) {
    const mediaStream = canvas.captureStream();
    console.log({mediaStream});
    return mediaStream;
}