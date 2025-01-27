package main

import "fmt"

func main() {

	var cedula string
	var costo float64
	var iva float64
	var montoTotal float64

	nombre := ""
	fmt.Println("Ingrese su nombre:")
	fmt.Scanln(&nombre)
	fmt.Println("Ingrese la cédula:")
	fmt.Scanln(&cedula)
	fmt.Println("Costo de los productos:")
	fmt.Scanln(&costo)

	iva = costo * 0.16
	montoTotal = costo + iva

	fmt.Println(" ")
	fmt.Println("FACTURA")
	fmt.Println(" ")
	fmt.Println("Cliente:", nombre)
	fmt.Println("Cédula de identidad:", cedula)
	fmt.Println(" ")
	fmt.Println("Monto:", costo)
	fmt.Println("IVA(16%)", iva)
	fmt.Println(" ")
	fmt.Println("Monto Total:", montoTotal)
}
