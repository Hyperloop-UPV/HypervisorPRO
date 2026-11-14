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
      /*
        PARADO = 0,
        LEVITANDO = 1,
        FUNCIONANDO = 2,
        SIMULATION = 3
        */
      //agregar escenas aquí
      //
      //
      //MAQUETA
        PARADO = 0,
        TEST = 1,
        LEVITANDO = 2,
        OSCILANDO = 3,
        FULLVEL = 4

    }


    [Header("REFERENCES")]

    [SerializeField]
    private Transform podPosition;

    [SerializeField]
    private CameraController cameraController;

    [SerializeField]
    private GameObject railT;
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


    [Space (10)]
    [Header("REAL SIMULATION DATA")]

    [SerializeField]
    private string providerIP;

    [SerializeField]
    private int providerPort;

    [SerializeField]
    private int refreshRate_ms; // El tiempo que tarda en solicitar nueva informacion

    private UDPClient udpClient;

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

        udpClient = GetComponent<UDPClient>();
        udpClient.ConnectToProvider(providerIP, providerPort, refreshRate_ms); // SOLO PARA LA MAQUETA

        StartCoroutine(EnterSimulationLoop());
    }

    /*
    public void TransitionToNewScene(int sceneId)
    {
        StartCoroutine(HandleNewScene(sceneId));
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
            case SceneNames.SIMULATION:
                StartCoroutine(SimulationSceneLoop());
                break;
          
        }
    }
    */


    // FOR MAQUETA ONLY
    private IEnumerator EnterSimulationLoop()
    {
      while (true) {
        yield return StartCoroutine(HandleScene_maqueta((SceneNames) udpClient.liveData.demo_id));
      }
    }


    private IEnumerator HandleScene_maqueta(SceneNames sceneId)
    {

        cameraController.TransitionToNewscene((int)udpClient.liveData.demo_id);
        switch (sceneId)
        {
          case SceneNames.PARADO:
            while (udpClient.liveData.demo_id == 0){

              yield return new WaitForSeconds(.1f);
            }
            break;
          case SceneNames.TEST:
            yield return StartCoroutine(SimulationDemo_test());
            break;
          case SceneNames.LEVITANDO:
            yield return StartCoroutine(SimulationDemo_levitation());
            break;
          case SceneNames.OSCILANDO:
            yield return StartCoroutine(SimulationDemo_oscilation());
            break;
          case SceneNames.FULLVEL:
            yield return StartCoroutine(SimulationDemo_fullSpeed());
            break;
        }


    }

    // SCENES //
    
    // Every scene is a subrutine (runs on the same thread but uses the unity scheduler) that
    // that forks its behaviour depending on its id. 
    // 
    // There are two bool variables (lockInScene and lockExitedScene) that acts as locks in 
    // order to keep the sincronous flow of data between scenes.
    //
    //  - lockInScene locks the scene until a new event calls the function TransitionToNewScene()
    //    and if lockInScene = false, new events will be discarted.
    //
    //  - lockExitedScene locks the scene until the exit rutine is completed, this allows the last
    //    scene to complete before a new scene is set

    
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

        lockInScene = true;
        while (lockInScene)
        {
            yield return null;
        }
        lockExitedScene = true;
    }
  
    private IEnumerator SimulationSceneLoop()
    {
        StartCoroutine(LerpToPos(mockPodInitialPosition));

        udpClient.ConnectToProvider(providerIP, providerPort, refreshRate_ms);
        lockInScene = true;
        while (lockInScene)
        {
          switch (udpClient.liveData.demo_id)
          {
            case 0 : /*Debug.Log("No demo");*/ break;
            case 1 : SimulationDemo_test(); break;
          }
          yield return new WaitForSeconds(refreshRate_ms / 1000f);
        }
        udpClient.DissconectFromProvider();
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








    // MAQUETA ONLY
    
    private IEnumerator SimulationDemo_test()
    {
      uint current_scene = udpClient.liveData.demo_id;
      while (current_scene == udpClient.liveData.demo_id)
      {
        Debug.Log($"Velocity: [{udpClient.liveData.speed_kmh}]\nAcceleration: [{udpClient.liveData.acceleration_kmh}]\nLevitation_mm[{udpClient.liveData.levitation_mm}]");
        yield return null;
      }
    
      yield return StartCoroutine(LerpToPos(mockPodInitialPosition));

    }



    private IEnumerator SimulationDemo_levitation()
    {
      uint current_scene = udpClient.liveData.demo_id;
      while (current_scene == udpClient.liveData.demo_id)
      {
        podPosition.position = new Vector3(0.0f, mockPodInitialPosition.y + udpClient.liveData.levitation_mm/1000.0f, 0.0f);
        yield return null;
      }

      yield return StartCoroutine(LerpToPos(mockPodInitialPosition));
    }



    private IEnumerator SimulationDemo_oscilation()
    {
      uint current_scene = udpClient.liveData.demo_id;
  
      yield return StartCoroutine(LerpToPos(new Vector3(mockPodInitialPosition.x, mockPodInitialPosition.y, mockPodInitialPosition.z + 15.0f)));

      // check that velocity is positive (accelerating) 
      while (udpClient.liveData.demo_id == current_scene) 
      {
        float last_velocity = udpClient.liveData.speed_kmh;
        yield return new WaitForSeconds(0.1f);
        if (last_velocity - udpClient.liveData.speed_kmh < 0)
        {
          break;
        }
      }


      while (current_scene == udpClient.liveData.demo_id)
      {
        float curr_position_offset = podPosition.position.z + udpClient.liveData.speed_kmh * Time.deltaTime;
        Debug.Log($"movement Offset x :[{curr_position_offset}]");
        podPosition.position = new Vector3(0.0f, mockPodInitialPosition.y + udpClient.liveData.levitation_mm/1000.0f, curr_position_offset);
        yield return null;
      }

      yield return StartCoroutine(LerpToPos(mockPodInitialPosition));
    }

    private IEnumerator SimulationDemo_fullSpeed(){
      uint current_scene = udpClient.liveData.demo_id;

      while (udpClient.liveData.demo_id == current_scene)
      {

        yield return null;
      }

      yield return StartCoroutine(LerpToPos(mockPodInitialPosition));

    }
}
