package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	Lectura := bufio.NewReader(os.Stdin)

	// Ejemplo 1: Convertir entero a cadena
	fmt.Println("Ingresa un numero:")
	texto1, _ := Lectura.ReadString('\n')
	numero, err := strconv.Atoi(strings.TrimSpace(texto1))
	if err != nil {
		fmt.Println("Error al convertir la cadena a entero:", err)
	} else {
		s := strconv.Itoa(numero)
		fmt.Println("Entero convertido a cadena:", s)
	}

	// Ejemplo 2: Convertir cadena a entero
	fmt.Println("Ingresa una cadena que representa un entero:")
	texto2, _ := Lectura.ReadString('\n')
	numero2, err := strconv.Atoi(strings.TrimSpace(texto2))
	if err != nil {
		fmt.Println("Error al convertir la cadena a entero:", err)
	} else {
		fmt.Println("Cadena convertida a entero:", numero2)
	}

	// Ejemplo 3: Convertir float a cadena
	fmt.Println("Ingresa un número flotante:")
	texto3, _ := Lectura.ReadString('\n')
	float, err := strconv.ParseFloat(strings.TrimSpace(texto3), 64)
	if err != nil {
		fmt.Println("Error al convertir la cadena a float:", err)
	} else {
		strFloat := strconv.FormatFloat(float, 'f', 2, 64)
		fmt.Println("Float convertido a cadena:", strFloat)
	}

	// Ejemplo 4: Convertir cadena a float
	fmt.Println("Ingresa una cadena que representa un float:")
	texto4, _ := Lectura.ReadString('\n')
	float2, err := strconv.ParseFloat(strings.TrimSpace(texto4), 64)
	if err != nil {
		fmt.Println("Error al convertir la cadena a float:", err)
	} else {
		fmt.Println("Cadena convertida a float:", float2)
	}

	// Ejemplo 5: Convertir booleano a cadena
	fmt.Println("Ingresa un booleano (true o false):")
	texto5, _ := Lectura.ReadString('\n')
	boolValor, err := strconv.ParseBool(strings.TrimSpace(texto5))
	if err != nil {
		fmt.Println("Error al convertir la cadena a booleano:", err)
	} else {
		strBool := strconv.FormatBool(boolValor)
		fmt.Println("Booleano convertido a cadena:", strBool)
	}
}
