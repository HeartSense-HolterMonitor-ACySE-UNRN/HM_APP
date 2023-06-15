function clickText1() {
	TESTER = document.getElementById('tester');

	invoke('grafico')
		.then((response) => {
			if (response.e === "") {
				TESTER.innerHTML = "";
				var data = [{
					x: response.x,
					y: response.y,
					line: {
						color: "firebrick",
                        width: 2
					}
				}];
				var layout = {
					title: 'Holter Monitor',
					xaxis: {
						title: 'seg',
						showgrid: true,
					},
					yaxis: {
						title: 'mV',
					}
					
				};
				Plotly.newPlot(TESTER, data, layout);
			} else {
				TESTER.innerHTML = response.e;
			}
		});
}

