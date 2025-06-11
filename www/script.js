// Dark mode button
document.getElementById('darkmode').addEventListener('click', () => 
    document.body.classList.toggle('dark')
);
// Navbar toggle
document.getElementById('toggle').addEventListener('click', () => 
    document.querySelector('#toggle > span').innerHTML = document.querySelector('nav > ul').classList.toggle('showed') ? 'close' : 'menu'
);