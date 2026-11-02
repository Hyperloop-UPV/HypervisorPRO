using UnityEngine;
using UnityEngine.Events;
using UnityEngine.InputSystem;

/*
*
*   Input controller es el controlador global que se encargar de detectar los inputs
*   del usuario para cambiar de escena.
*
*
*
*
*
*
*/

public class InputController : MonoBehaviour
{
    [SerializeField]
    UnityEvent<int> transitionToNewScene;

    private int currentScene;


    private CameraController cameraController;
    // Update is called once per frame

    void OnEnable()
    {
        cameraController = transform.GetComponent<CameraController>();
        currentScene = 0;
    }


    void Update()
    {
        if(!cameraController.lockInTransition)
        {
            HandleInputs();
        }
    }


    private void HandleInputs()
    {
        Keyboard keyboard = Keyboard.current;
            if (keyboard != null)
                HandleKeyboardInput(keyboard);
    }


    private void HandleKeyboardInput(Keyboard keyboard)
    {
        
        // Para debuggear solo, eliminar luego
        if (keyboard.digit0Key.wasPressedThisFrame)
        {
            currentScene = 0;
            transitionToNewScene?.Invoke(currentScene);
        }
        if (keyboard.digit1Key.wasPressedThisFrame)
        {
            currentScene = 1;
            transitionToNewScene?.Invoke(currentScene);
        }
        if (keyboard.digit2Key.wasPressedThisFrame)
        {
            currentScene = 2;
            transitionToNewScene?.Invoke(currentScene);
        }




        if (keyboard.aKey.wasPressedThisFrame || keyboard.leftArrowKey.wasPressedThisFrame)
        {
            if(currentScene <= 0)
                currentScene = cameraController.sceneCount-1; 
            else
                currentScene--;
            
            transitionToNewScene?.Invoke(currentScene);
        }

        if (keyboard.dKey.wasPressedThisFrame || keyboard.rightArrowKey.wasPressedThisFrame)
        {
            if(currentScene >= cameraController.sceneCount-1)
                currentScene = 0; 
            else
                currentScene++;
            
            transitionToNewScene?.Invoke(currentScene);
        }

    }
}
