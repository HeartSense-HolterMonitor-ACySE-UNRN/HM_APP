document.addEventListener('DOMContentLoaded', function() {
	const contentDiv = document.getElementById('content');

	function loadContent(url) {
		fetch(url)
			.then(response => response.text())
			.then(data => {
				contentDiv.innerHTML = data;
			})
			.catch(error => {
				console.log('Error loading content:', error);
			});
	}

	// Event listeners for menu buttons
	document.getElementById('estado').addEventListener('click', function() {
		loadContent('estado.html');
	});
	
	document.getElementById('holter').addEventListener('click', function() {
		loadContent('holter.html');
	});

	document.getElementById('inicio').addEventListener('click', function() {
		loadContent('inicio.html');
	});

	document.getElementById('borrar').addEventListener('click', function() {
		loadContent('borrar.html');
	});


	document.getElementById('ayuda').addEventListener('click', function() {
		loadContent('ayuda.html');
	});

});
