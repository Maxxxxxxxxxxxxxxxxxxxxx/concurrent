import sysv_ipc
import time

klucz = 11

try:
    sem = sysv_ipc.Semaphore(klucz, sysv_ipc.IPC_CREX,0o700,1)
    # robiÄ tu inne rzeczy, ktĂłre ma zrobiÄ tylko proces ktĂłry wygraĹ wyscig
    pierwszy = True    
except sysv_ipc.ExistentialError:
    # drugi proces juĹź utworzyĹ semafor
    sem = sysv_ipc.Semaphore(klucz)
    pierwszy=False
    time.sleep(0.1)    
    # czekam chwilÄ, aĹź pierwszy proces skoĹczy to co ma zrobiÄ jako pierwszy

if pierwszy:
    print('jestem pierwszy')
    time.sleep(5)    
    sem.remove()
else:
    print('jestem drugi')
