package main

import (
    "bufio"
    "fmt"
    "log"
    "os"
    "unicode"
    "strconv"
)

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}

	scanner := bufio.NewScanner(file)

    sum := 0
	for scanner.Scan() {
        line := scanner.Text()

        calibration_numbers := make([]int, 2)
        for _, letter := range line {
           if unicode.IsDigit(letter) {
               str := string(letter)
                if calibration_numbers[0] == 0 {
                    calibration_numbers[0], _ = strconv.Atoi(str)
                    calibration_numbers[1] = calibration_numbers[0]
                } else {
                    calibration_numbers[1], _ = strconv.Atoi(str)
                }
           }
        }
        sum += calibration_numbers[0]*10 +  calibration_numbers[1]
	}
    fmt.Println(sum)
    
    if err := scanner.Err(); err != nil {
		log.Fatal(err)
	}
    
	file.Close()
}
