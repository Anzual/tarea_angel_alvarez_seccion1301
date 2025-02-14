package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// CuentaVocales cuenta el número de vocales en una cadena
func CuentaVocales(s string) int {
	vocales := "aeiouAEIOU"
	contador := 0
	for _, char := range s {
		if strings.ContainsRune(vocales, char) {
			contador++
		}
	}
	return contador
}

// CuentaConsonantes cuenta el número de consonantes en una cadena
func CuentaConsonantes(s string) int {
	vocales := "aeiouAEIOU"
	contador := 0
	for _, char := range s {
		if !strings.ContainsRune(vocales, char) && ((char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z')) {
			contador++
		}
	}
	return contador
}

// TienePrefijo verifica si una cadena tiene un prefijo específico
func TienePrefijo(s, prefijo string) bool {
	return strings.HasPrefix(s, prefijo)
}

// TieneSufijo verifica si una cadena tiene un sufijo específico
func TieneSufijo(s, sufijo string) bool {
	return strings.HasSuffix(s, sufijo)
}

// ConcatenarCadenas concatena dos cadenas
func ConcatenarCadenas(s1, s2 string) string {
	s2 = "Extra"
	return s1 + s2
}

func main1() {
	Lectura := bufio.NewReader(os.Stdin)
	fmt.Println("Ingresa una oracion:")
	texto, _ := Lectura.ReadString('\n')
	fmt.Println("Original:", texto)
	fmt.Println("CuentaVocales:", CuentaVocales(texto))
	fmt.Println("CuentaConsonantes:", CuentaConsonantes(texto))
	fmt.Println("TienePrefijo (comprobar 'Hola'):", TienePrefijo(texto, "Hola"))
	fmt.Println("TieneSufijo (comprobar 'mundo'):", TieneSufijo(texto, "mundo"))
	fmt.Println("ConcatenarCadenas (añadir ' extra'):", ConcatenarCadenas(texto, "extra"))
	
}
