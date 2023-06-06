class HeartMonitor {
    private heartRateElement: HTMLSpanElement;
    private pulseElement: HTMLDivElement;
    private heartRate: number;
  
    constructor() {
      this.heartRateElement = document.getElementById("heartRate") as HTMLSpanElement;
      this.pulseElement = document.getElementById("pulse") as HTMLDivElement;
      this.heartRate = 0;
    }
  
    // Setter method for heart rate
    setHeartRate(rate: number): void {
      this.heartRate = rate;
      this.updateHeartRateElement();
    }
  
    // Private method to update the heart rate element
    private updateHeartRateElement(): void {
      this.heartRateElement.textContent = this.heartRate.toString();
    }
  }
  
  // Create an instance of HeartMonitor class
  const monitor = new HeartMonitor();
  
  // Example usage
  monitor.setHeartRate(80);
  