package main

import (
	"fmt"
	"log"
	"strings"

	"github.com/hslam/ipc"
)

const (
	SEMAPHORE_KEY      = 12
	A_KEY              = 3333
	B_KEY              = 4444
	SHARED_MEMORY_SIZE = 1024
)

func main() {

	isPlayerOne := true

	var playerOneMemoryId int
	var playerOneData []byte

	var playerTwoMemoryId int
	var playerTwoData []byte

	semId, err := ipc.Semget(SEMAPHORE_KEY, 2, ipc.IPC_CREAT|ipc.IPC_EXCL|0666)

	if err != nil {
		semId, _ = ipc.Semget(SEMAPHORE_KEY, 2, 0)
		isPlayerOne = false

		playerOneMemoryId, playerOneData, err = ipc.Shmgetattach(A_KEY, SHARED_MEMORY_SIZE, 0)
		if err != nil {
			log.Panic(err)
		}

		playerTwoMemoryId, playerTwoData, err = ipc.Shmgetattach(B_KEY, SHARED_MEMORY_SIZE, 0)
		if err != nil {
			log.Panic(err)
		}
	} else {
		ipc.Semsetvalue(semId, 0, 1)
		ipc.Semsetvalue(semId, 1, 0)

		playerOneMemoryId, playerOneData, err = ipc.Shmgetattach(A_KEY, SHARED_MEMORY_SIZE, ipc.IPC_CREAT|ipc.IPC_EXCL|0666)
		if err != nil {
			log.Panic(err)
		}

		playerTwoMemoryId, playerTwoData, err = ipc.Shmgetattach(B_KEY, SHARED_MEMORY_SIZE, ipc.IPC_CREAT|ipc.IPC_EXCL|0666)
		if err != nil {
			log.Panic(err)
		}
	}

	defer ipc.Semrm(semId)
	defer ipc.Shmrm(playerOneMemoryId)
	defer ipc.Shmdetach(playerOneData)
	defer ipc.Shmrm(playerTwoMemoryId)
	defer ipc.Shmdetach(playerTwoData)

	selfSemNum := 0
	otherSemNum := 1

	if isPlayerOne {
		fmt.Println("jestes graczem 1.")
	} else {
		selfSemNum, otherSemNum = otherSemNum, selfSemNum
		fmt.Println("jestes graczem 2.")
	}

	score := 0
	for i := range [3]int{} {
		fmt.Printf("\ntura: %d\n", i+1)

		ipc.Semp(semId, selfSemNum, ipc.SEM_UNDO)

		fmt.Print("wprowadz wybor (a/b/c): ")

		var choice string
		fmt.Scan(&choice)
		choice = strings.ToLower(choice)

		if isPlayerOne {
			copy(playerOneData, choice)
		} else {
			copy(playerTwoData, choice)
		}

		ipc.Semv(semId, otherSemNum, ipc.SEM_UNDO)
		ipc.Semp(semId, selfSemNum, ipc.SEM_UNDO)

		secondChoice := string(playerTwoData[:1])
		if !isPlayerOne {
			secondChoice = string(playerOneData[:1])
		}

		ipc.Semv(semId, otherSemNum, ipc.SEM_UNDO)

		fmt.Printf("wybrales: %s, przeciwnik wybral: %s\n", choice, secondChoice)

		if ((secondChoice != choice) && isPlayerOne) || ((secondChoice == choice) && !isPlayerOne) {
			score++
			fmt.Println("wygrales runde!")
		} else {
			fmt.Println("przegrales runde!")
		}

		fmt.Printf("wynik: %d\n", score)
	}

	fmt.Printf("\nwynik koncowy: %d", score)
}
