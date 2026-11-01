using UnityEngine;


[CreateAssetMenu(fileName = "SceneData", menuName = "Scriptable Objects/SceneData")]
public class SceneData : ScriptableObject
{

    public string     sceneName;
    public bool       freeCam;

    public Vector3    trainStartPos;
    public Vector3    cameraStartPos;
    public Vector3    cameraFocusPoint;

}
