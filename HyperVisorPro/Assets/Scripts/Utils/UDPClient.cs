using UnityEngine;
using System.Threading;
using System.Collections.Concurrent;
using System.Net;
using System.Net.Sockets;
using System.Text;

public class UDPClient : MonoBehaviour
{

  public struct PodData {

    public float speed;
    public float levitation;

  }

  [HideInInspector]
  public PodData liveData;

  private UdpClient client;
  private IPEndPoint providerEndPoint;
  private Thread recieveThread;

  private ConcurrentQueue<byte> messajes = new ConcurrentQueue<byte>();

  public void ConnectToProvider(string ip, int port, int refreshRate_ms)
  {
        client = new UdpClient();
        providerEndPoint = new IPEndPoint(IPAddress.Parse(ip), port);

        recieveThread = new Thread(() => updateInfoLoop(client,refreshRate_ms)) { IsBackground = true };
        recieveThread.Start();
  }

  private void updateInfoLoop(UdpClient _client, int refreshRate_ms)
  {
      // mientras el cliente exista y sea valido
      while (_client == client && client != null){

        byte[] data = new byte[0];
        client.Send(data, data.Length, providerEndPoint);
        byte[] rcv_data = client.Receive(ref providerEndPoint);

        Thread.Sleep(refreshRate_ms);
      }
  }

  public void DissconectFromProvider()
  {
        client?.Close();
        client = null;
        recieveThread?.Join(500);
  }

  public void OnDestroy()
  {
        client?.Close();
        client = null;
  }

}
