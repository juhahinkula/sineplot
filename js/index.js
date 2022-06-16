import("../pkg").then(module => {
  const freqInput = document.getElementById('freq');
  const amplInput = document.getElementById('ampl');
  const drawBtn = document.getElementById('draw');

  drawBtn.addEventListener('click', () => {
      const amplitude = parseFloat(amplInput.value) || 0;
      const frequency = parseFloat(freqInput.value) || 0;
      module.draw(amplitude, frequency);
  });

  module.draw(20, 20);
});
