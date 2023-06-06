class HeartSenseMonitor {
    private heartRateElement: HTMLSpanElement;
    public pulseElement: HTMLDivElement;
    private heartRate: number;
  
    constructor(heartRateElement: HTMLSpanElement, pulseElement: HTMLDivElement) {
      this.heartRateElement = heartRateElement;
      this.pulseElement = pulseElement;
      this.heartRate = 0;
    }
  
    setHeartRate(heartRate: number) {
      this.heartRate = heartRate;
      this.heartRateElement.innerText = heartRate.toString();
      this.pulseElement.style.opacity = '1';
    }
  }
  
  const heartRateElement = document.getElementById('heartRate') as HTMLSpanElement;
  const pulseElement = document.querySelector('.pulse') as HTMLDivElement;
  const heartSenseMonitor = new HeartSenseMonitor(heartRateElement, pulseElement);
  
  // Simulate heart rate readings
  setInterval(() => {
    const randomHeartRate = Math.floor(Math.random() * 100) + 60;
    heartSenseMonitor.setHeartRate(randomHeartRate);
    setTimeout(() => {
      heartSenseMonitor.pulseElement.style.opacity = '0';
    }, 500);
  }, 2000);
  
  const helpIcon = document.getElementById("helpIcon") as HTMLElement;
  
  helpIcon.addEventListener("click", function() {
    window.open("help.html", "_blank");
  });
  
  function handleMenuItemClick(event: Event) {
    event.preventDefault();
    const menuItem = (event.target as HTMLElement).innerText;
  
    // Handle different menu item actions here
    switch (menuItem) {
      case 'Estado del Holter':
        // Code for handling 'Estado del Holter' menu item
        break;
      case 'Borrar SD':
        // Code for handling 'Borrar SD' menu item
        break;
      case 'Iniciar Holter':
        // Code for handling 'Iniciar Holter' menu item
        break;
      case 'Analizar Datos':
        // Code for handling 'Analizar Datos' menu item
        break;
      default:
        break;
    }
  }
  
  const menuItems = document.getElementsByClassName('navbar-menu-item');
  Array.from(menuItems).forEach((menuItem) => {
    menuItem.addEventListener('click', handleMenuItemClick);
  });
  