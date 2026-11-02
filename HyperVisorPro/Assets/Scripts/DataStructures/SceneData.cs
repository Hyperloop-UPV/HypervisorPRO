using UnityEngine;


[CreateAssetMenu(fileName = "SceneData", menuName = "Scriptable Objects/SceneData")]
public class SceneData : ScriptableObject
{
    public int      sceneId;
    public string   sceneName;
    public float    cameraDistanceToFocusPoint;
    [Range(60.0f,120.0f)]
    public float    cameraFOV;

    [Range(0.0f,360.0f)]
    public float    camearAngleX;
    
    [Range(0.0f,360.0f)]
    public float    cameraAngleY;
  

}
