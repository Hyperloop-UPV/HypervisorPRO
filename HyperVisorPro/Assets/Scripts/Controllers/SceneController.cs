using System.Collections;
using Unity.VisualScripting;
using UnityEditorInternal;
using UnityEngine;

public class SceneController : MonoBehaviour
{
   // Describe el nombre de las escenas (para entender mejor el código)
   // Si se añade una nueva escena, debe ser agregado su id aquí
    private enum SceneNames
    {
        PARADO = 0,
        LEVITANDO = 1,
        FUNCIONANDO = 2,
        
        // agregar escenas aquí 
    }


    [Header("REFERENCES")]
    
    [SerializeField]
    private Transform podPosition;

    [Space(10)]
    [Header("MOCK SIMULATION DATA")]

    [SerializeField] [Range (0.0f, 100.0f)]
    private float mockLevitationOffset_mm; 

    [SerializeField] [Range (1.0f, 150.0f)]
    private float mockLevitationOscilationOffset_mm;

    [SerializeField] [Range (1.0f, 10.0f)]
    private float mockLevittionOscilationSpeed;
    
    [SerializeField] [Range(0.1f, 5.0f)]
    private float mockLevitationInitializationTime;

    
    [SerializeField]
    private float   mockPodMaxVelocity;
    
    [SerializeField]
    private float   mockPodAcceleration;

    [SerializeField]
    private bool lockInScene;
    private bool lockExitedScene;
    private int   currentSceneId;
    private float mockPodVelocity;
    private Vector3 mockPodInitialPosition;


    public void OnEnable()
    {
        lockInScene     = false;
        lockExitedScene = true;
        currentSceneId  = -1; 
        mockPodVelocity = 0.0f;

        mockPodInitialPosition = new Vector3(podPosition.position.x, podPosition.position.y, podPosition.position.z);

        TransitionToNewScene(0);
    }

    public void TransitionToNewScene(int sceneId)
    {
        StartCoroutine(HandleNewScene(sceneId));
    }

    private void HandleScene(SceneNames sceneId)
    {
        switch (sceneId)
        {
            case SceneNames.PARADO:
                StartCoroutine(StoppedSceneLoop());
                break;
            case SceneNames.LEVITANDO:
                StartCoroutine(levitatingSceneLoop());
                break;
            case SceneNames.FUNCIONANDO:
                StartCoroutine(RunningSceneLoop());
                break;
        }
    }


    private IEnumerator HandleNewScene(int sceneId)
    {

        // unlok scene and wait for last scene to end
        lockInScene = false;
        while (!lockExitedScene){
            yield return new WaitForSeconds(.1f);
        }
        lockExitedScene = false;

        // start new scene
        HandleScene((SceneNames) sceneId);
        currentSceneId = sceneId;

    }


    private IEnumerator StoppedSceneLoop()
    {
        StartCoroutine(LerpToPos(mockPodInitialPosition));
        lockInScene = true;
        // Este bucle se ejecutará hasta que una nueva escena sea invocada
        while (lockInScene)
        {
            yield return null;
        }
        lockExitedScene = true;
    }

    private IEnumerator levitatingSceneLoop()
    {
        Vector3 levitatingPosition = new Vector3(mockPodInitialPosition.x, mockPodInitialPosition.y + mockLevitationOffset_mm/1000.0f, mockPodInitialPosition.z);
        StartCoroutine(LerpToPos(levitatingPosition));

        float timer = 0.0f;
        Vector3 oscilationOffset =  new Vector3(0.0f,0.0f,0.0f);

        lockInScene = true;
        while (lockInScene)
        {
            timer += Time.deltaTime;
            oscilationOffset.y = Mathf.Cos(timer * mockLevittionOscilationSpeed) * mockLevitationOscilationOffset_mm/1500000;
            podPosition.position += oscilationOffset;

            yield return null;
        }
        lockExitedScene = true;
    }

    private IEnumerator RunningSceneLoop()
    {
        Vector3 levitatingPosition = new Vector3(mockPodInitialPosition.x, mockPodInitialPosition.y + mockLevitationOffset_mm/1000, mockPodInitialPosition.z);
        
        if (levitatingPosition != podPosition.position)
            StartCoroutine(LerpToPos(levitatingPosition));
    
        while (lockInScene)
        {
            yield return null;
        }
        lockExitedScene = true;
    }


    private IEnumerator LerpToPos(Vector3 targetPos)
    {
        Vector3 initialPosition = podPosition.position;
        float elapsedTime = 0.0f;
        float progress;

        while (elapsedTime <= mockLevitationInitializationTime)
        {   
            elapsedTime += Time.deltaTime;
            progress = elapsedTime/mockLevitationInitializationTime; // lineal progress but can be changed to exponential or other functions

            podPosition.position = Vector3.Lerp(initialPosition, targetPos, progress);
            yield return null;
        }
    }
}
