using System.Runtime.InteropServices;
using UnityEngine;

public class CommunicationController : MonoBehaviour
{
    [DllImport ("Udp_comm")]
    private static extern uint add(uint left, uint right);

    void OnEnable()
    {
        uint result = add(1, 2);
        Debug.Log(result);
    }

    void FixedUpdate()
    {

    }
}
