using UnityEngine;

public class CameraController : MonoBehaviour
{
    private bool f_inTransition;
    private bool f_freeCam;

    [SerializeField] 
    private GameObject mainCamera;

    [SerializeField]
    private SceneData[] sceneSettings;

    private string currentScene;


    // Start is called once before the first execution of Update after the MonoBehaviour is created
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        
    }


    /*
    *   StartTransitionNewscene() recibe un string identificador desde un evento y empieza la transición a la
    *   nueva escena. setea f_inTransition a true mientras esté en escena, flag la cual actua como
    *   lock, lo cual impide empezar otra transición si otro evento es recibido.
    *   
    *   La función busca la escena en el array sceneSettings el nombre recibido, y de encontrarlo carga la configuración
    *   en escena (posición del tren, posición de la cámara, etc). Además configura el scene index a la posición de la 
    *   escena dentro del array.
    */
    public  void StartTransitionNewscene(string sceneName) {}

    private void EnterNewScene(string currentScene){} // cambiar los strings  por hashes

    private void OscilateTargetToRight(float degrees, float t){}

    private void OscilateTargetToLeft(float degrees, float t){}

    private void LerpToPos(Vector3 targetPos, float t){}

}
