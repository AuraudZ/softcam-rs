void *__cdecl scCreateCamera(int width, int height, float framerate);
void __cdecl scDeleteCamera(void *camera);

void __cdecl scSendFrame(void *camera, const void *image_bits);

int __cdecl scWaitForConnection(void *camera, float timeout);

int __cdecl scIsConnected(void *camera);
