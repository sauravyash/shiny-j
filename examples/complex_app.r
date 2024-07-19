library(shiny)
library(ggplot2)
library(dplyr)

# Define UI
ui <- fluidPage(
    titlePanel("Advanced Shiny App: Data Explorer"),
    sidebarLayout(
        sidebarPanel(
            fileInput("file", "Choose CSV File",
                      accept = c("text/csv", "text/comma-separated-values,text/plain", ".csv")),
            checkboxInput("header", "Header", TRUE),
            radioButtons("sep", "Separator",
                         choices = c(Comma = ",", Semicolon = ";", Tab = "\t"),
                         selected = ","),
            selectInput("plotType", "Select Plot Type",
                        choices = c("Scatter" = "scatter", "Histogram" = "hist", "Box" = "box")),
            uiOutput("dynamicInputs"),
            actionButton("updatePlot", "Update Plot"),
            downloadButton("downloadPlot", "Download Plot")
        ),
        mainPanel(
            tabsetPanel(
                tabPanel("Plot", plotOutput("plot")),
                tabPanel("Summary", verbatimTextOutput("summary")),
                tabPanel("Data", DT::dataTableOutput("table"))
            )
        )
    )
)

# Define server logic
server <- function(input, output, session) {
    # Reactive value to store the dataset
    data <- reactiveVal()

    # Read CSV file
    observeEvent(input$file, {
        req(input$file)
        df <- read.csv(input$file$datapath, header = input$header, sep = input$sep)
        data(df)
    })

    # Dynamic UI for selecting variables
    output$dynamicInputs <- renderUI({
        req(data())
        switch(input$plotType,
               "scatter" = list(
                   selectInput("xvar", "X Variable", choices = names(data())),
                   selectInput("yvar", "Y Variable", choices = names(data()))
               ),
               "hist" = selectInput("var", "Variable", choices = names(data())),
               "box" = list(
                   selectInput("var", "Variable", choices = names(data())),
                   selectInput("group", "Group By", choices = c("None", names(data())))
               )
        )
    })

    # Generate plot
    plot <- eventReactive(input$updatePlot, {
        req(data())
        p <- switch(input$plotType,
               "scatter" = ggplot(data(), aes_string(x = input$xvar, y = input$yvar)) +
                   geom_point() + geom_smooth(method = "lm"),
               "hist" = ggplot(data(), aes_string(x = input$var)) +
                   geom_histogram(bins = 30),
               "box" = {
                   if(input$group != "None") {
                       ggplot(data(), aes_string(x = input$group, y = input$var)) + geom_boxplot()
                   } else {
                       ggplot(data(), aes_string(y = input$var)) + geom_boxplot()
                   }
               }
        )
        p + theme_minimal() + labs(title = "Data Visualization")
    })

    # Render plot
    output$plot <- renderPlot({
        plot()
    })

    # Render summary
    output$summary <- renderPrint({
        req(data())
        summary(data())
    })

    # Render data table
    output$table <- DT::renderDataTable({
        req(data())
        DT::datatable(data())
    })

    # Download handler for the plot
    output$downloadPlot <- downloadHandler(
        filename = function() { paste("plot", input$plotType, "png", sep = ".") },
        content = function(file) {
            ggsave(file, plot = plot(), device = "png")
        }
    )
}

# Run the application
shinyApp(ui = ui, server = server)