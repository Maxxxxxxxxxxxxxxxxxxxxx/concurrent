# import sysv_ipc

# klucz = 2137

# SHARED_MEMORY_KEY = 42
# SHARED_MEMORY_SIZE = 1024

# key = sysv_ipc.ftok("/", SHARED_MEMORY_KEY)

# try:
#     sem = sysv_ipc.Semaphore(klucz, sysv_ipc.IPC_CREX,0o700,1)

#     print("Gracz 1 odpalił grę!")

    

# except:
    

import sysv_ipc

SMEM_KEY1 = 42
SMEM_KEY2 = 43
SHARED_MEMORY_SIZE = 1024
SEMAPHORE_KEY = 2137
NULL_CHAR = '\0'

def cleanup_shm(shm):
    shm.detach()
    shm.remove()

def get_mem(key):
    sysv_ipc.SharedMemory(key, sysv_ipc.IPC_CREX | 0o600, size=SHARED_MEMORY_SIZE)

def is_empty(mem):
    s = mem.read()
    s = s.decode()
    i = s.find(NULL_CHAR)
    if i == -1:
        return True
    else:
        return False

def main():
    # Generate a key for the shared memory segment
    shm_key = sysv_ipc.ftok("/", SMEM_KEY1)
    shm_key2 = sysv_ipc.ftok("/", SMEM_KEY2)
    sem_key = sysv_ipc.ftok("/", SEMAPHORE_KEY)

    # Player 1
    try:
        # Shared memory 1
        shm1 = get_mem(shm_key)
        sem = sysv_ipc.Semaphore(sem_key, sysv_ipc.IPC_CREX, 0o700,1)

        print("Player 1 started")

        playeroption = 0

        while True:
            playeroption = input()

            if playeroption in [1,2,3]:
                shm1.write(playeroption.encode())
                break
        
        p2mem = get_mem(shm_key2)

        while True:
            if not is_empty(p2mem):
                data = p2mem.read()
                data = data.decode()

                print(f"Player 2's choice: {data}")

                if int(data) == playeroption:
                    print("Player 2 won!")
                    break
                



    except sysv_ipc.ExistentialError:
        # Shared memory 2
        shm2 = get_mem(shm_key2)

        playeroption = 0

        while True:
            playeroption = input()

            if playeroption in [1,2,3]:
                shm1.write(playeroption.encode())
                break

        shm = sysv_ipc.SharedMemory(shm_key)

        print("Player 2 started")



    cleanup_shm(shm1)
    cleanup_shm(shm2)

    sysv_ipc.remove_semaphore(sem)

    print("Deleted shared memory segment and semaphore")

if __name__ == "__main__":
    main()
