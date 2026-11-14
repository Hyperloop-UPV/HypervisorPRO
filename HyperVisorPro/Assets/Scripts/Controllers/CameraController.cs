using System.Collections;
using UnityEngine;

public class CameraController : MonoBehaviour
{
    [Header("REFERENCES")]

    [SerializeField] 
    private GameObject mainCamera;

    [SerializeField]
    private Transform podPosition;

    [Space(10)]
    [Header("Camera Settings for each scene")]

    [SerializeField]
    private SceneData[] sceneSettings;
    

    [Space(10)]
    [Header("MODIFIERS")]

    [SerializeField]
    private float transitionTime;


    // SHARED variables (with InputController)
    [HideInInspector]
    public bool lockInTransition;
    [HideInInspector]
    public int sceneCount;
    
    // private variables
    private Vector3 cameraFocusPoint;

    
    void OnEnable()
    {
        lockInTransition = false;

        sceneCount       = sceneSettings.Length;
        cameraFocusPoint = podPosition.position;

        //TransitionToNewscene(0);
    }

    /*
    *   TransitionNewscene() recibe un identificador desde un evento y empieza la transición a la
    *   nueva escena. setea lockInTransition a true mientras esté en escena, flag la cual actua como
    *   candado que impide enviar nuevos eventos de cambio de transición desde el inputControler.
    *   
    *   La función busca la escena en el array sceneSettings el nombre recibido, y de encontrarlo carga la configuración
    *   en escena (posición del tren, posición de la cámara, etc). Además configura el scene index a la posición de la 
    *   escena dentro del array.
    */
    public void TransitionToNewscene(int sceneId)
    {
        lockInTransition = true;  // se bloquea la transición

        int sceneIndex = findSceneSettings(sceneId);

        //lerp to new position
        StartCoroutine(OscilateToPos(
            sceneSettings[sceneIndex].cameraAngleY,
            sceneSettings[sceneIndex].camearAngleX,
            sceneSettings[sceneIndex].cameraDistanceToFocusPoint,
            sceneSettings[sceneIndex].cameraFOV
            ));

    }



    // Función auxiliar que busca el indice de la configuración de la escena dentro del array (sceneSettings)
    // dado el identificador sceneID
    private int findSceneSettings(int sceneId)
    {
        // Find settings index in the sceneSettings array
        int sceneIndex = 0; 
        foreach(SceneData data in sceneSettings)
        {
            if (data.sceneId == sceneId)
                break;  
            sceneIndex++;
        }
        return sceneIndex;
    }

    private Vector3 CalculateCameraPosition(float angleY, float angleX, float distance)
    {
        Vector3 destVec = new Vector3();
        float radY = angleY * Mathf.Deg2Rad;
        float radX = angleX * Mathf.Deg2Rad;

        destVec.x = Mathf.Cos(radX) * Mathf.Sin(radY);
        destVec.y = Mathf.Sin(radX);
        destVec.z = Mathf.Cos(radX) * Mathf.Cos(radY);

        return destVec * distance;
    }


    private IEnumerator LerpToPos(Vector3 targetPos)
    {
        Vector3 initialPosition = mainCamera.transform.position;

        float elapsedTime = 0.0f;
        float progress;

        while (elapsedTime <= transitionTime)
        {            
            elapsedTime += Time.deltaTime;
            progress = elapsedTime/transitionTime; // lineal progress but can be changed to exponential or other functions

            mainCamera.transform.position = Vector3.Lerp(initialPosition, targetPos, progress);

            yield return null;
        }
        lockInTransition = false;
    }


    /*
    * Esta función, que es derivada de LerpToPos(), hace una interpolación lineal entre dos puntos,
    * pero en lugar de seguir una linea recta, realiza la interpolación de los ángulos,
    * que se deducen mediante cálculos cartesianos para resolver cual debe ser la posición de la cámara (mainCamera)
    * dados unos angulos X e Y, y una distancia hacia el foco (cameraFocusPoint).
    */ 

    private IEnumerator OscilateToPos(float targetAngleY, float targetAngleX, float targetDist, float targetFOV)
    {
        float elapsedTime = 0.0f;
        float progress;

        float initialDist   = Vector3.Distance(mainCamera.transform.position, cameraFocusPoint);
        float currentDist;

        Camera cameraReference = mainCamera.transform.GetComponent<Camera>();
        float initialFOV = cameraReference.fieldOfView;
        
        Quaternion initialRotation = mainCamera.transform.rotation;
        Quaternion targetRotation = Quaternion.Euler(targetAngleX, targetAngleY, 0.0f);

        while (elapsedTime <= transitionTime)
        {
            elapsedTime += Time.deltaTime;
            progress = elapsedTime/transitionTime; // lineal progress but can be changed to exponential


            currentDist = Mathf.Lerp(initialDist, targetDist, progress);
       
            Quaternion currentRotation = Quaternion.Slerp(initialRotation, targetRotation, progress);
            
            Vector3 dir = currentRotation * Vector3.forward;
            mainCamera.transform.position = cameraFocusPoint - (dir * currentDist);
            mainCamera.transform.rotation = Quaternion.LookRotation(
                cameraFocusPoint - mainCamera.transform.position,
                Vector3.up
            );

            cameraReference.fieldOfView = Mathf.Lerp(initialFOV, targetFOV, progress );


            yield return null;
        }

        lockInTransition = false; // unlocks so that it can transition again
    }
}
