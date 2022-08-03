import matplotlib.pyplot as plt
import soundfile


def autocorrelation(signal, lag=1, window_size=10):
    result = []
    for i in range(len(signal)):
        s = 0
        for j in range(window_size):
            x1 = signal[i + j] if i + j < len(signal) else 0.0
            x2 = signal[i + j + lag] if i + j + lag < len(signal) else 0.0
            s += x1 * x2
        result.append(s)
    return result


def main():
    input_file, _sample_rate = soundfile.read("../../../../../input-files/piano-a440.wav")
    input_file = [x[0] for x in input_file[0:400]]

    fig = plt.figure()

    axs = fig.add_subplot(projection="3d")
    for i in range(0, len(input_file), 20):
        acf = autocorrelation(input_file, lag=i)
        x_data = [x for x in range(len(input_file))]
        y_data = [i for _j in range(len(input_file))]
        z_data = [acf[z] for z in range(len(input_file))]
        axs.plot(x_data, y_data, z_data, "b-", alpha=0.8)

    x_data = [i for i in range(len(input_file))]
    y_data = [0 for _j in range(len(input_file))]
    z_data = [input_file[j] for j in range(len(input_file))]
    axs.plot(x_data, y_data, z_data, "r")

    plt.show()


if __name__ == "__main__":
    main()
