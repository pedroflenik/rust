import pandas as pd
import matplotlib.pyplot as plt


# 0 para grafico médio | 1 para grafico ganho
tipoGrafico = 0

if tipoGrafico == 0:

    df = pd.read_csv('stored_time.txt')

    avg_df = df.groupby('threads')['real_seconds'].mean().reset_index()

    plt.figure(figsize=(9, 5))
    plt.plot(avg_df['threads'], avg_df['real_seconds'], marker='o', linestyle='-', color='b', linewidth=2)

    plt.title('Atividade Crivo tempo médio')
    plt.xlabel('Número de Threads')
    plt.ylabel('Tempo médio')
    plt.grid(True, linestyle='--', alpha=0.6)
    plt.xticks(avg_df['threads'])
    plt.tight_layout()

    plt.savefig('crivo_medio.png', dpi=300)
    print("Grafico salvo em crivo_medio.png")

else:
    df = pd.read_csv('stored_time.txt')
    avg_df = df.groupby('threads')['real_seconds'].mean().reset_index()

    t1 = avg_df.loc[avg_df['threads'] == 1, 'real_seconds'].values[0]

    avg_df['speedup'] = t1 / avg_df['real_seconds']

    ideal_speedup = avg_df['threads']

    plt.figure(figsize=(9, 5))
    plt.plot(avg_df['threads'], avg_df['speedup'], marker='o', linestyle='-', color='b', label='Ganho obtido', linewidth=2)
    plt.plot(avg_df['threads'], ideal_speedup, linestyle='--', color='r', label='Ganho esperado', linewidth=1.5)


    plt.title('Atividade Primos em paralelo ganho')
    plt.xlabel('Numero de threads')
    plt.ylabel('Ganho')
    plt.grid(True, linestyle='--', alpha=0.6)
    plt.legend()
    plt.xticks(avg_df['threads'])
    plt.tight_layout()

    plt.savefig('crivo_ganho.png', dpi=300)
    print("Grafico csalvvo e  crivo_ganho.png")