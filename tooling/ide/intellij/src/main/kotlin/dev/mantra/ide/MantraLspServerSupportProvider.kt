package dev.mantra.ide

import com.intellij.execution.configurations.GeneralCommandLine
import com.intellij.openapi.project.Project
import com.intellij.openapi.vfs.VirtualFile
import com.intellij.platform.lsp.api.LspServerSupportProvider
import com.intellij.platform.lsp.api.ProjectWideLspServerDescriptor

class MantraLspServerSupportProvider : LspServerSupportProvider {
    override fun fileOpened(
        project: Project,
        file: VirtualFile,
        serverStarter: LspServerSupportProvider.LspServerStarter
    ) {
        if (file.extension == "mantra") {
            serverStarter.ensureServerStarted(MantraLspServerDescriptor(project))
        }
    }
}

class MantraLspServerDescriptor(project: Project) : ProjectWideLspServerDescriptor(project, "Mantra") {
    override fun isSupportedFile(file: VirtualFile): Boolean = file.extension == "mantra"

    override fun createCommandLine(): GeneralCommandLine {
        return GeneralCommandLine("mantra-lsp", "--stdio")
    }
}

