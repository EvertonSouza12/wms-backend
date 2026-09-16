# 3. Etapa de Build (SDK completa)
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
WORKDIR /src

# Copia e restaura as dependências primeiro (otimização de cache)
COPY ["WmsBackend.csproj", "./"]
RUN dotnet restore "WmsBackend.csproj"

# Copia o código restante e compila
COPY . .
RUN dotnet publish "WmsBackend.csproj" -c Release -o /app/publish /p:UseAppHost=false

# 2. Etapa Final (Apenas Runtime leve)
FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS final
WORKDIR /app
COPY --from=build /app/publish .

# Define a porta padrão do ASP.NET 8+
EXPOSE 8080
ENV ASPNETCORE_HTTP_PORTS=8080

ENTRYPOINT ["dotnet", "WmsBackend.dll"]