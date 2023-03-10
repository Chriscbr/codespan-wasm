const rust = import('./pkg');

rust
  .then(m => {
    let x = m.greet('World!');
    console.log(x);
  })
  .catch(console.error);
